use lif::Neuron;

fn main() {
    let mut neuron = Neuron::new(0.0, 0.01, -10.0, -60.0, 2.0);
    neuron.lif();

    println!("Results: {:?}", neuron.result);
}