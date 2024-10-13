use std::path::PathBuf;
use std::process::Command;
use rusqlite::{params, Connection, Result};
use tauri::Manager;

// Get the path to store the database
fn get_db_path() -> PathBuf {
    let app_dir = tauri::api::path::app_dir(&tauri::Config::default())
        .expect("Failed to get app directory");
    app_dir.join("vpn_paths.db")
}

// Initialize the SQLite database (if it doesn't exist)
fn initialize_db() -> Result<()> {
    let db_path = get_db_path();
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS vpn_paths (
                  id INTEGER PRIMARY KEY,
                  path TEXT NOT NULL
                  )",
        [],
    )?;
    Ok(())
}

// Add a new VPN path to the database
#[tauri::command]
fn add_vpn_path(path: String) -> Result<(), String> {
    let db_path = get_db_path();
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    conn.execute("INSERT INTO vpn_paths (path) VALUES (?1)", params![path])
        .map_err(|e| e.to_string())?;
    Ok(())
}

// Delete all VPN paths from the database
#[tauri::command]
fn delete_vpn_paths() -> Result<(), String> {
    let db_path = get_db_path();
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM vpn_paths", []).map_err(|e| e.to_string())?;
    Ok(())
}

// Fetch all VPN paths from the database
#[tauri::command]
fn get_vpn_paths() -> Result<Vec<String>, String> {
    let db_path = get_db_path();
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT path FROM vpn_paths").map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| Ok(row.get(0)?)).map_err(|e| e.to_string())?;

    let mut paths = Vec::new();
    for path in rows {
        paths.push(path.map_err(|e| e.to_string())?);
    }
    Ok(paths)
}

// Function to turn VPN on using stored paths
#[tauri::command]
fn vpn_on() -> Result<String, String> {
    println!("Attempting to turn VPN ON...");
    let paths = get_vpn_paths()?;

    let mut output_msg = String::new();
    for path in paths {
        let output = Command::new("sudo")
            .arg("wg-quick")
            .arg("up")
            .arg(path.clone())
            .output();

        match output {
            Ok(o) if o.status.success() => {
                output_msg.push_str(&format!("VPN ON for {}: Success\n", path));
            }
            Ok(o) => {
                output_msg.push_str(&format!("VPN ON for {}: Failed - {}\n", path, String::from_utf8_lossy(&o.stderr)));
            }
            Err(e) => {
                output_msg.push_str(&format!("VPN ON for {}: Error - {}\n", path, e));
            }
        }
    }
    Ok(output_msg)
}

// Function to turn VPN off using stored paths
#[tauri::command]
fn vpn_off() -> Result<String, String> {
    println!("Attempting to turn VPN OFF...");
    let paths = get_vpn_paths()?;

    let mut output_msg = String::new();
    for path in paths {
        let output = Command::new("sudo")
            .arg("wg-quick")
            .arg("down")
            .arg(path.clone())
            .output();

        match output {
            Ok(o) if o.status.success() => {
                output_msg.push_str(&format!("VPN OFF for {}: Success\n", path));
            }
            Ok(o) => {
                output_msg.push_str(&format!("VPN OFF for {}: Failed - {}\n", path, String::from_utf8_lossy(&o.stderr)));
            }
            Err(e) => {
                output_msg.push_str(&format!("VPN OFF for {}: Error - {}\n", path, e));
            }
        }
    }
    Ok(output_msg)
}

fn main() {
    // Initialize the database on startup
    initialize_db().expect("Failed to initialize the database");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![add_vpn_path, delete_vpn_paths, get_vpn_paths, vpn_on, vpn_off])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
