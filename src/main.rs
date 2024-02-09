mod prediction;
mod discord;
mod constants;

fn main() {
    // TODO: recreate model on a timer or via command...
    // prediction::recreate_model(6000);
    discord::main();
}
