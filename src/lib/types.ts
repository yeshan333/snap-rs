export interface DockApp {
  name: string;
  bundle_id: string;
  app_path: string;
  icon_base64: string | null;
  shortcut_key: string | null;
}

export interface AppSettings {
  shortcuts_enabled: boolean;
  autostart_enabled: boolean;
}
