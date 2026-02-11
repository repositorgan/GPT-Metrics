import { invoke } from "@tauri-apps/api/tauri";

window.addEventListener("DOMContentLoaded", () => {
  invoke("track_app_open");
});
