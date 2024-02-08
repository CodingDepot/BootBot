mod prediction;
mod discord;

fn main() {
    // TODO: recreate model on a timer or via command...
    // prediction::recreate_model(3000);
    discord::main();
}
