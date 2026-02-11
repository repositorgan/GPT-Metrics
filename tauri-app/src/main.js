import { invoke } from "@tauri-apps/api/tauri";

window.addEventListener("DOMContentLoaded", () => {
  invoke("track_event", { eventType: "app_open" });
});
