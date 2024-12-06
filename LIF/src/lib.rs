#[allow(non_snake_case)]

 pub struct Neuron {
    t:f64, //initial time
    dt:f64,//time step 
   // c:f64, //capacitance
   // i_leak:f64, //leak current 
    v:f64, //membrane potential 
    u:f64, //resting voltage its the voltage the neuron settles
    tau:f64, //time constant
   // i_spikes:f64, //spike current  
    //i:f64,   //injected or driving current
   pub result:Vec<f64>,  
    
}

impl Neuron {  pub fn new(t: f64, dt: f64, v: f64, u: f64, tau: f64) -> Self {
    Self {
        t,
        dt,
        v,
        u,
        tau,
        result: Vec::new(),
    }
}
   pub fn lif (&mut self)  {
   

    while self.t < 10.0 {
        self.result.push(self.v);
        let dv = -(self.v - self.u) / self.tau;

        
        self.v += dv * self.dt;
        self.t += self.dt;

}

}
}
#[allow(unused_imports)]

mod tests {
    use super::*;

    #[test]
    fn test_neuron() {
        // Test if the neuron is initialized correctly
        let neuron = Neuron::new(0.0, 0.01, -10.0, -60.0, 2.0);
        assert_eq!(neuron.t, 0.0);
        assert_eq!(neuron.dt, 0.01);
        assert_eq!(neuron.v, -10.0);
        assert_eq!(neuron.u, -60.0);
        assert_eq!(neuron.tau, 2.0);
        assert!(neuron.result.is_empty()); // Result vector should start empty
    }

    #[test]

    fn neuron_simulation (){
        let mut neuron = Neuron::new(0.0, 0.01, -10.0, -60.0, 2.0);
        neuron.lif();
        assert!(!neuron.result.is_empty()); 
        assert_eq!(neuron.result.len(), 1001);
        assert_eq!(neuron.result[0], -10.0);
        let last_values: Vec<f64> = neuron.result.iter().rev().take(5).cloned().collect();
        println!("Last values: {:?}", last_values);
        let last_value = *neuron.result.last().unwrap();
        println!("y:{:?}",last_value - neuron.u.abs()<1e-2);
        assert!((last_value - neuron.u.abs() < 1e-2));
    }   
    }