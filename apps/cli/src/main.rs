use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;

use ray_tracer_core::canvas::Canvas;
use ray_tracer_core::scenes;

fn main() -> std::io::Result<()> {
    let canvas = prompt_screne();

    println!("Enter output file name (without extension): ");
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name)?;
    let file_name = file_name.trim();

    let file_name = if file_name.is_empty() {
        "render"
    } else {
        file_name
    };

    let output_dir = PathBuf::from("renders");
    fs::create_dir_all(&output_dir)?;

    let file_path = output_dir.join(format!("{file_name}.ppm"));
    fs::write(&file_path, canvas.to_ppm())?;

    println!("Saved to: {}", file_path.display());

    if let Some(path_str) = file_path.to_str() {
        try_open_file(path_str);
    }

    Ok(())
}

/// Prompt for all available rendered scenes.
fn prompt_screne() -> Canvas {
    loop {
        println!("Choose a scene:");
        println!("1 - Projectile");
        println!("2 - Clock");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(choice) = input.trim().parse::<u32>() {
            if let Some(scene) = get_scene(choice) {
                return scene;
            }
        }
        println!("Invalid choice\n");
    }
}

/// Return the canvas for the selected scene.
fn get_scene(choice: u32) -> Option<Canvas> {
    match choice {
        1 => Some(scenes::render_projectile()),
        2 => Some(scenes::render_clock()),
        _ => None,
    }
}

/// Try to open the generated ppm file.
pub fn try_open_file(path: &str) {
    if let Err(e) = Command::new("xdg-open").arg(path).spawn() {
        eprintln!("Could not open file automatically: {}", e);
    }
}
