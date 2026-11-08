(() => {
  'use strict';
  const runtime = document.getElementById('runtime');
  const status = document.getElementById('status');
  const monitorSelect = document.getElementById('monitor');
  const refreshButton = document.getElementById('refresh');
  const captureButton = document.getElementById('capture');
  const screen = document.getElementById('screen');
  const placeholder = document.getElementById('placeholder');
  const dimensions = document.getElementById('dimensions');
  const native = window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke;
  function message(text, kind) { status.textContent = text; status.className = `status ${kind || ''}`; }
  if (!native) {
    runtime.textContent = 'Browser mode';
    message('Native capture is unavailable in a regular browser. Run the Tauri desktop app to request screen capture. This page does not simulate a capture.', 'error');
    return;
  }
  runtime.textContent = 'Native app · local capture';
  refreshButton.disabled = false;
  captureButton.disabled = false;
  async function loadMonitors() {
    refreshButton.disabled = true;
    monitorSelect.disabled = true;
    message('Requesting available display information from the native backend…');
    try {
      const monitors = await native('list_monitors');
      monitorSelect.replaceChildren();
      if (!monitors.length) {
        monitorSelect.add(new Option('No displays found', ''));
        message('The native backend did not find a display.', 'error');
        return;
      }
      for (const monitor of monitors) {
        const label = `${monitor.name} · ${monitor.width}×${monitor.height}${monitor.isPrimary ? ' · Primary' : ''}`;
        monitorSelect.add(new Option(label, monitor.id));
      }
      monitorSelect.disabled = false;
      message(`${monitors.length} display${monitors.length === 1 ? '' : 's'} available. Capture is local and only starts when you press the button.`, 'success');
    } catch (error) {
      message(String(error), 'error');
    } finally { refreshButton.disabled = false; }
  }
  async function capture() {
    if (!monitorSelect.value) return;
    captureButton.disabled = true;
    message('Capturing the selected display on this computer…');
    try {
      const frame = await native('capture_monitor', { monitorId: monitorSelect.value });
      screen.src = frame.dataUrl;
      screen.hidden = false;
      placeholder.hidden = true;
      dimensions.textContent = `${frame.width} × ${frame.height}`;
      message('Capture complete. The image is displayed locally and has not been transmitted.', 'success');
    } catch (error) {
      message(String(error), 'error');
    } finally { captureButton.disabled = false; }
  }
  refreshButton.addEventListener('click', loadMonitors);
  captureButton.addEventListener('click', capture);
  loadMonitors();
})();
