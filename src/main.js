const { invoke } = window.__TAURI__.tauri;

async function loadVpnPaths() {
  const paths = await invoke("get_vpn_paths");
  const vpnPathsList = document.getElementById("vpn-paths-list");
  vpnPathsList.innerHTML = ""; // Clear the list
  paths.forEach(path => {
    const li = document.createElement("li");
    li.textContent = path;
    vpnPathsList.appendChild(li);
  });
}

document.getElementById("add-path").addEventListener("click", async () => {
  const path = document.getElementById("vpn-path").value;
  if (path) {
    await invoke("add_vpn_path", { path });
    document.getElementById("vpn-path").value = ""; // Clear the input
    loadVpnPaths(); // Refresh the list
  }
});

document.getElementById("delete-paths").addEventListener("click", async () => {
  await invoke("delete_vpn_paths");
  loadVpnPaths(); // Refresh the list
});

document.getElementById("vpn-on").addEventListener("click", async () => {
  const terminal = document.getElementById("terminal");
  terminal.innerHTML = "Turning VPN ON...\n";
  const output = await invoke("vpn_on");
  terminal.innerHTML += output;
});

document.getElementById("vpn-off").addEventListener("click", async () => {
  const terminal = document.getElementById("terminal");
  terminal.innerHTML = "Turning VPN OFF...\n";
  const output = await invoke("vpn_off");
  terminal.innerHTML += output;
});

// Load the VPN paths on page load
window.addEventListener("DOMContentLoaded", loadVpnPaths);
