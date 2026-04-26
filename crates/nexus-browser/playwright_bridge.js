const { chromium } = require('playwright');
const fs = require('fs');

async function run() {
    const args = process.argv.slice(2);
    const specIndex = args.indexOf('--spec-json');
    if (specIndex === -1) {
        console.error("Missing --spec-json");
        process.exit(1);
    }
    const spec = JSON.parse(args[specIndex + 1]);

    const browser = await chromium.launch({ headless: true });
    const context = await browser.newContext();
    const page = await context.newPage();

    let summary = "";
    let textSnippet = "";
    const transcript = ["Browser launched", "Context created"];

    try {
        if (spec.target_url) {
            transcript.push(`Navigating to ${spec.target_url}`);
            await page.goto(spec.target_url, { waitUntil: 'networkidle' });
            summary = `Successfully visited ${spec.target_url}. `;
        }

        if (spec.intent === 'extract_information') {
            transcript.push("Extracting page text content");
            textSnippet = await page.innerText('body');
            textSnippet = textSnippet.substring(0, 1000) + "...";
            summary += "Information extracted from the page.";
        } else {
            summary += "Page loaded successfully.";
        }

    } catch (err) {
        transcript.push(`Error: ${err.message}`);
        summary = `Failed to complete browser task: ${err.message}`;
    }

    await browser.close();

    const output = {
        summary,
        transcript,
        target_url: spec.target_url,
        intent: spec.intent,
        mode: spec.mode,
        action_phase: spec.action_phase,
        boundary: "standard browser observation",
        text_snippet: textSnippet,
        link_sample: [],
        recommended_next_actions: ["Analyze the extracted text", "Navigate to a sub-page if needed"]
    };

    console.log(JSON.stringify(output));
}

run();
