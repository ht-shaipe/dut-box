async function init() {
  const loadingEl = document.getElementById('loading');
  const appEl = document.getElementById('app');

  try {
    // Import the WASM module
    const wasm = await import('./wasm/dut_box_web.js');
    await wasm.default();

    // 等待中文字体加载完成后再初始化，避免 canvas 内中文乱码
    await document.fonts.ready;
    try {
      await document.fonts.load('16px "Noto Sans SC"');
    } catch (_) {}

    // Initialize the app
    await wasm.init_app('canvas');

    // Hide loading indicator
    if (loadingEl) {
      loadingEl.remove();
    }
  } catch (error) {
    console.error('Failed to initialize:', error);

    // Show error message
    if (loadingEl) {
      loadingEl.innerHTML = `
        <div class="error">
          <h2>Failed to load the application</h2>
          <p>${error.message || error}</p>
          <p style="margin-top: 10px; font-size: 14px;">
            Please check the console for more details.
          </p>
        </div>
      `;
    }
  }
}

init();
