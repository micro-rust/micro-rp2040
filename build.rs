//! `micro-rp2040` build script.
//! Copies the linker to a LLVM accessible location and compiles the necessary stage 2 bootloader.

use std::env;
use std::fs::{ self, File };
use std::io::{ self, Read, Write };
use std::path::{ PathBuf };
use std::process::Command;

#[cfg(feature = "w25q080")]
static NAME : &'static str = "w25q080";

#[cfg(feature = "w25x10cl")]
static NAME : &'static str = "w25x10cl";

#[cfg(not(feature = "flash-defined"))]
static NAME : &'static str = "generic03h";


fn main() {
	// Get the paths to all the files.
	let (asm, elf, bin, rust) = buildpaths();

	// Compile the source code.
	compile(&asm, &elf);

	// Extract .text section.
	extract(&elf, &bin);

	// Create Rust module.
	rustify(&bin, &rust);
}


fn rustify(bin: &PathBuf, rust: &PathBuf) {
	// Read in the .text section.
	let mut text = fs::read(bin).expect("Could not pad binary. Could not open binary file.");

	// Check that the .text section has legal size.
	if text.len() > 252 {
		panic!("Generated .text section exceeds 252 bytes.");
	}

	// Fill with 0 until 252 bytes.
	text.extend(&vec![0u8; 252 - text.len()]);

	// Calculate CRC word.
	let crc = crc(&text);

	// Push CRC into .text.
	text.extend(crc.to_le_bytes());

	// Create Rust source code.
	let mut code = String::new();

	code.push_str(&format!(
"//! Rust module for the {} Flash second stage bootloader.

#[link_section = \".bootloader\"]
#[used]
pub static CODE : [u8; 256] = [\n
", NAME));

	for i in 0..16 {
		code.push_str("\n    ");
		for j in 0..16 {
			code.push_str(&format!("{:02x}, ", text[(i * 16) + j]));
		}
	}

	code.push_str("\n];\n");

	// Create the Rust file.
	let parent = rust.parent().expect("Could not create Rust file parent directory tree.");
	fs::create_dir_all(parent).expect("Could not create Rust directory tree.");

	let mut file = File::open(rust).expect("Could not open Rust module file.");

	write!(file, "{}", code).expect("Could not write to Rust file.");
}

fn extract(elf: &PathBuf, bin: &PathBuf) {
	// Command to extract .text section.
	let extraction = Command::new("arm-none-eabi-objcopy")
		.arg("-O")
		.arg("binary")
		.arg(elf)
		.arg(bin)
		.output()
		.expect("Could not extract .text section. Command build failed.");

	io::stderr().write_all(&extraction.stderr).unwrap();

	if !extraction.status.success() {
		panic!("Could not extract .text section. Objcopy failed.");
	}
}

fn compile(asm: &PathBuf, elf: &PathBuf) {
	// Command to compile the stage 2 bootloader assembly.
	let compilation = Command::new("arm-none-eabi-gcc")
		.arg("-nostartfiles")
		.arg("-fPIC")
		.arg("--specs=nosys.specs")
		.arg(asm)
		.arg("-o")
		.arg(elf)
		.output()
		.expect("Could not compile stage 2 bootloader. Command creation failed.");

	io::stderr().write_all(&compilation.stderr).unwrap();

	if !compilation.status.success() {
		panic!("Could not compile stage 2 bootloader. GCC failed.")
	}
}

fn buildpaths() -> (PathBuf, PathBuf, PathBuf, PathBuf) {
	// Get output directory.
	let outdir = PathBuf::from( env::var("OUT_DIR").expect("Could not get output directory.") );

	// Get current directory.
	let curdir = env::current_dir().expect("Coudl not get current directory.");

	// Build assembly path.
	let asm = curdir.join("src").join("sys").join("boot2").join("src").join(&format!("{}.S", NAME));

	// Build ELF path.
	let elf = outdir.join(&format!("{}.elf", NAME));

	// Build binary path.
	let bin = outdir.join(&format!("{}.bin", NAME));

	// Build Rust path.
	let rust = curdir.join("src").join("sys").join("boot2").join(NAME).join("mod.rs");

	(asm, elf, bin, rust)
}

fn crc(input: &[u8]) -> u32 {
	let mut engine = crc_any::CRCu32::crc32mpeg2();
	engine.digest(input);
	engine.get_crc()
}