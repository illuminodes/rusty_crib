export async function clipboardCopy(text) {
    try {
        await navigator.clipboard.writeText(text);
    } catch (err) {
        console.error("Failed to copy text to clipboard:", err);
    }
}
