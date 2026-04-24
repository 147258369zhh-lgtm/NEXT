#!/usr/bin/env node

function parseArgs(argv) {
  const args = [...argv];
  let specJson = null;

  while (args.length > 0) {
    const current = args.shift();
    if (current === "--spec-json") {
      specJson = args.shift() ?? null;
    }
  }

  return {
    specJson: specJson ?? process.env.NEXUS_BROWSER_SPEC_JSON ?? null
  };
}

function buildFallback(spec) {
  const targetUrl = spec.target_url ?? null;
  const boundary = getIntentBoundary(spec.intent);
  return {
    summary: [
      "Browser bridge worker received the task.",
      "",
      `Mode: ${spec.mode}`,
      `Intent: ${spec.intent}`,
      `Risk: ${spec.risk_level}`,
      `Target URL: ${targetUrl ?? "not detected"}`,
      "Forms detected: 0",
      `Boundary: ${boundary}`,
      "",
      "Current behavior: this repository-local bridge is wired correctly, but it is still running in scaffold mode.",
      "Next step: replace the internal fallback with real Playwright navigation and structured extraction."
    ].join("\n"),
    transcript: [
      "browser bridge worker started",
      `mode=${spec.mode}`,
      `intent=${spec.intent}`,
      targetUrl ? `target_url=${targetUrl}` : "target_url=not_detected",
      `boundary=${boundary}`,
      "executed scaffold fallback path"
    ],
    target_url: targetUrl,
    intent: spec.intent,
    mode: spec.mode,
    action_phase: spec.action_phase,
    boundary,
    text_snippet: null,
    link_sample: [],
    form_count: 0,
    input_sample: [],
    field_plan: [],
    missing_fields: [],
    sensitive_fields: [],
    recommended_next_actions: getRecommendedActions(spec.intent)
  };
}

async function maybeRunPlaywright(spec) {
  const bridgeMode = process.env.NEXUS_BROWSER_BRIDGE_MODE ?? "fallback";
  if (bridgeMode !== "playwright") {
    return null;
  }

  try {
    const playwright = await import("playwright");
    const browser = await playwright.chromium.launch({
      headless: spec.mode !== "takeover"
    });
    const page = await browser.newPage();

    if (spec.target_url) {
      await page.goto(spec.target_url, {
        waitUntil: "domcontentloaded",
        timeout: 15000
      });
    }

    const title = await page.title();
    const url = page.url();
    const extraction = await collectStructuredData(page, spec.intent);
    const boundary = getIntentBoundary(spec.intent);
    await browser.close();

    return {
      summary: [
        "Browser bridge worker executed the Playwright path.",
        "",
        `Mode: ${spec.mode}`,
        `Intent: ${spec.intent}`,
        `Resolved URL: ${url}`,
        `Page title: ${title || "[empty title]"}`,
        `Boundary: ${boundary}`,
        extraction.textSnippet ? `Content snippet: ${extraction.textSnippet}` : null,
        `Forms detected: ${extraction.formCount}`,
        extraction.inputSample.length > 0
          ? `Input sample: ${extraction.inputSample.join(", ")}`
          : null,
        extraction.linkSample.length > 0
          ? `Link sample: ${extraction.linkSample.join(", ")}`
          : null
      ]
        .filter(Boolean)
        .join("\n"),
      transcript: [
        "browser bridge worker started",
        "playwright runtime selected",
        spec.target_url ? `navigated_to=${spec.target_url}` : "no_target_url_provided",
        `resolved_url=${url}`,
        `title=${title || "[empty title]"}`,
        `boundary=${boundary}`,
        extraction.textSnippet
          ? "structured_text_extraction=ok"
          : "structured_text_extraction=empty",
        `structured_forms=${extraction.formCount}`,
        extraction.inputSample.length > 0
          ? `structured_inputs=${extraction.inputSample.length}`
          : "structured_inputs=0",
        extraction.linkSample.length > 0
          ? `structured_links=${extraction.linkSample.length}`
          : "structured_links=0"
      ],
      target_url: url,
      intent: spec.intent,
      mode: spec.mode,
      action_phase: spec.action_phase,
      boundary,
      text_snippet: extraction.textSnippet || null,
      link_sample: extraction.linkSample,
      form_count: extraction.formCount,
      input_sample: extraction.inputSample,
      field_plan: extraction.fieldPlan,
      missing_fields: extraction.missingFields,
      sensitive_fields: extraction.sensitiveFields,
      recommended_next_actions: getRecommendedActions(spec.intent)
    };
  } catch (error) {
    return {
      summary: [
        "Browser bridge worker attempted the Playwright path but failed.",
        "",
        `Reason: ${error instanceof Error ? error.message : String(error)}`,
        "",
        "Fallback behavior preserved the bridge contract, so the browser runtime remains callable."
      ].join("\n"),
      transcript: [
        "browser bridge worker started",
        "playwright runtime selected",
        `playwright_error=${error instanceof Error ? error.message : String(error)}`
      ],
      target_url: spec.target_url ?? null,
      intent: spec.intent,
      mode: spec.mode,
      action_phase: spec.action_phase,
      boundary: getIntentBoundary(spec.intent),
      text_snippet: null,
      link_sample: [],
      form_count: null,
      input_sample: [],
      field_plan: [],
      missing_fields: [],
      sensitive_fields: [],
      recommended_next_actions: getRecommendedActions(spec.intent)
    };
  }
}

async function collectStructuredData(page, intent) {
  const rawText = await page.evaluate(() => {
    const source =
      document.querySelector("main")?.innerText ??
      document.body?.innerText ??
      "";
    return source;
  });

  const textSnippet = rawText
    .replace(/\s+/g, " ")
    .trim()
    .slice(0, intent === "extract_information" ? 400 : 180);

  const linkSample = await page.evaluate(() => {
    return Array.from(document.querySelectorAll("a[href]"))
      .slice(0, 5)
      .map((anchor) => {
        const text = (anchor.textContent ?? "").replace(/\s+/g, " ").trim();
        const href = anchor.getAttribute("href") ?? "";
        return text ? `${text} -> ${href}` : href;
      })
      .filter(Boolean);
  });

  const { formCount, inputSample, fieldPlan, missingFields, sensitiveFields } =
    await page.evaluate(() => {
    const forms = Array.from(document.querySelectorAll("form"));
    const controls = Array.from(
      document.querySelectorAll("input, textarea, select, button")
    );
    const inputs = controls
      .slice(0, 6)
      .map((element) => {
        const tag = element.tagName.toLowerCase();
        const descriptor =
          element.getAttribute("type") ??
          element.getAttribute("name") ??
          element.getAttribute("placeholder") ??
          "";
        return descriptor ? `${tag}:${descriptor}` : tag;
      });
    const classifySensitive = (label, type) => {
      const lower = `${label} ${type}`.toLowerCase();
      return [
        "password",
        "passcode",
        "验证码",
        "code",
        "otp",
        "token",
        "secret",
        "phone",
        "email",
        "card",
        "bank"
      ].some((token) => lower.includes(token));
    };
    const fieldEntries = controls
      .filter((element) => {
        const tag = element.tagName.toLowerCase();
        return tag === "input" || tag === "textarea" || tag === "select";
      })
      .slice(0, 6)
      .map((element) => {
        const tag = element.tagName.toLowerCase();
        const name =
          element.getAttribute("name") ??
          element.getAttribute("id") ??
          element.getAttribute("placeholder") ??
          "unnamed";
        const type = element.getAttribute("type") ?? tag;
        const currentValue =
          "value" in element && typeof element.value === "string"
            ? element.value.trim()
            : "";
        return {
          label: name,
          type,
          missing: currentValue.length === 0,
          sensitive: classifySensitive(name, type)
        };
      });
    const fieldPlan = fieldEntries.map(
      (entry) => `${entry.label} => pending (${entry.type})`
    );
    const missingFields = fieldEntries
      .filter((entry) => entry.missing)
      .map((entry) => `${entry.label} (${entry.type})`);
    const sensitiveFields = fieldEntries
      .filter((entry) => entry.sensitive)
      .map((entry) => `${entry.label} (${entry.type})`);

    return {
      formCount: forms.length,
      inputSample: inputs,
      fieldPlan,
      missingFields,
      sensitiveFields
    };
  });

  return {
    textSnippet,
    linkSample,
    formCount,
    inputSample,
    fieldPlan,
    missingFields,
    sensitiveFields
  };
}

function getIntentBoundary(intent) {
  if (intent === "login") {
    return "inspect-first login flow; do not submit credentials automatically";
  }
  if (intent === "fill_form") {
    return "inspect-first form flow; do not submit automatically";
  }
  return "standard browser observation";
}

function getRecommendedActions(intent) {
  if (intent === "login") {
    return [
      "Inspect visible login fields and session hints.",
      "Confirm whether credentials should be provided in a controlled follow-up step.",
      "Do not submit credentials automatically."
    ];
  }
  if (intent === "fill_form") {
    return [
      "Map required fields and missing values.",
      "Prepare a field plan before any submission attempt.",
      "Do not submit the form automatically."
    ];
  }
  if (intent === "extract_information") {
    return [
      "Review the extracted snippet and sampled links.",
      "Decide whether deeper extraction is needed."
    ];
  }
  return [
    "Confirm the intended browser action.",
    "Choose the next controlled step."
  ];
}

async function main() {
  const { specJson } = parseArgs(process.argv.slice(2));
  if (!specJson) {
    throw new Error("missing --spec-json argument or NEXUS_BROWSER_SPEC_JSON env");
  }

  const spec = JSON.parse(specJson);
  const playwrightResult = await maybeRunPlaywright(spec);
  const result = playwrightResult ?? buildFallback(spec);

  process.stdout.write(JSON.stringify(result));
}

main().catch((error) => {
  const message = error instanceof Error ? error.message : String(error);
  process.stderr.write(message);
  process.exitCode = 1;
});
