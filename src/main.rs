mod check_active;
mod connectVPN;
mod exitWireGuardCli;
mod interaction;
//use nix::unistd::Uid;
fn main() {
    //   if !Uid::effective().is_root() {
    //     println!("Bitte als root ausführen (sudo)!");
    //   std::process::exit(1);
    // }
    interaction::interaction();
}
