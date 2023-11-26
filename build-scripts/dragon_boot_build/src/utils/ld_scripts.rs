use super::cargo_handler::{CargoHandler, TargetArch};

pub fn setup() {
    let arch = CargoHandler::target_arch();
    match arch {
        TargetArch::Riscv64 => {
            // CargoHandler::emit_link_arg("-Tsrc/arch/riscv64/link.ld");
            CargoHandler::emit_link_arg("-Tsrc/arch/riscv64/link.ld");
            CargoHandler::emit_link_arg("--emit-relocs");
            CargoHandler::emit_link_arg("--nmagic");

            CargoHandler::emit_link_arg("--no-relax");
        }
        _ => panic!("Unsupported arch: {:?}", arch),
    }
}
