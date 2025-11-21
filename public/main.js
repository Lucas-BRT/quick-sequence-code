import init, { init_app } from "./qsc_generator.js";

async function initializeApplication() {
  try {
    await init();
    init_app();
  } catch (error) {
    console.error("Failed to initialize WASM application:", error);

    const msgElement = document.getElementById("msg");
    if (msgElement) {
      msgElement.textContent = "INITIALIZATION FAILED";
      msgElement.style.color = "var(--accent-orange)";
      msgElement.classList.remove("loading");
    }
  }
}

initializeApplication();
