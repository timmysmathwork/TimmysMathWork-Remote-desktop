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
  const approvalStatus = document.getElementById('approval-status');
  const approvalDialog = document.getElementById('approval-dialog');
  const native = window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke;
  let activeRequest = null;
  let removeRequestListener = null;
  function message(text, kind) { status.textContent = text; status.className = `status ${kind || ''}`; }
  function showApproval(request) {
    if (!request || !request.requestId || activeRequest) return;
    activeRequest = request;
    document.getElementById('request-name').textContent = request.deviceName || 'Unknown device';
    document.getElementById('request-id').textContent = request.deviceId || 'Missing device ID';
    document.getElementById('request-os').textContent = request.operatingSystem || 'Unknown operating system';
    document.getElementById('request-access').textContent = Array.isArray(request.requestedAccess) ? request.requestedAccess.join(', ') : 'Not specified';
    approvalStatus.textContent = 'A connection request is awaiting your decision.';
    if (!approvalDialog.open) approvalDialog.showModal();
  }
  async function decide(approve) {
    if (!activeRequest) return;
    const request = activeRequest;
    activeRequest = null;
    document.getElementById('approve-request').disabled = true;
    document.getElementById('deny-request').disabled = true;
    try {
      await native('decide_connection_request', { requestId: request.requestId, approve });
      approvalStatus.textContent = approve ? 'Approved. No session starts until an authenticated transport is implemented.' : 'Request denied; no session was started.';
    } catch (error) {
      approvalStatus.textContent = `Request expired or could not be answered. Access was not approved. ${String(error)}`;
    } finally {
      document.getElementById('approve-request').disabled = false;
      document.getElementById('deny-request').disabled = false;
      if (approvalDialog.open) approvalDialog.close();
    }
  }
  document.getElementById('approve-request').addEventListener('click', () => decide(true));
  document.getElementById('deny-request').addEventListener('click', () => decide(false));
  approvalDialog.addEventListener('cancel', event => { event.preventDefault(); decide(false); });
  if (!native) {
    runtime.textContent = 'Browser mode';
    approvalStatus.textContent = 'Approval prompts require the native application; no incoming connection service is running.';
    message('Native capture and connection approval are unavailable in a regular browser. Run the Tauri app for local screen preview. This page does not simulate either capability.', 'error');
    return;
  }
  runtime.textContent = 'Native app · local capture';
  const listen = window.__TAURI__.event && window.__TAURI__.event.listen;
  if (listen) {
    listen('connection-request', event => showApproval(event.payload))
      .then(unlisten => { removeRequestListener = unlisten; })
      .catch(error => { approvalStatus.textContent = `Approval prompt listener failed; requests will not be authorized. ${String(error)}`; });
  } else {
    approvalStatus.textContent = 'Approval prompt listener unavailable; requests fail closed.';
  }
  window.addEventListener('beforeunload', () => { if (removeRequestListener) removeRequestListener(); });
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
