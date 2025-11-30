use crate::structtox::DNAencoder;
use crate::tox::read_fasta;
use std::collections::HashMap;
use std::error::Error;
use std::vec;
use tch::Device::Cpu;
use tch::{Device, Tensor, nn, nn::Module, nn::OptimizerConfig};

/*
Gaurav Sablok
codeprog@icloud.com
*/

#[derive(Debug)]
pub struct Autoencoder {
    encoder: nn::Sequential,
    decoder: nn::Sequential,
}

impl Autoencoder {
    fn toxencoder(vs: &nn::Path, input_dim: i64, bottleneck_dim: i64) -> Self {
        let encoder = nn::seq()
            .add(nn::linear(vs / "enc1", input_dim, 128, Default::default()))
            .add(nn::func(|xs| xs.relu()))
            .add(nn::linear(
                vs / "enc2",
                128,
                bottleneck_dim,
                Default::default(),
            ));
        let decoder = nn::seq()
            .add(nn::linear(
                vs / "dec1",
                bottleneck_dim,
                128,
                Default::default(),
            ))
            .add(nn::func(|xs| xs.relu()))
            .add(nn::linear(vs / "dec2", 128, input_dim, Default::default()));
        Self { encoder, decoder }
    }
}

impl Module for Autoencoder {
    fn forward(&self, xs: &Tensor) -> Tensor {
        let encoded = self.encoder.forward(xs);
        self.decoder.forward(&encoded)
    }
}

impl DNAencoder {
    pub fn run_encoder(
        &self,
        fastafile: &str,
        inputdim: i64,
        bottleneck: i64,
        epochs: i64,
    ) -> Result<String, Box<dyn Error>> {
        let traindata = encode(fastafile);
        let device = Device::Cpu;
        let vs = nn::VarStore::new(device);
        let autoencoder = Autoencoder::toxencoder(&vs.root(), inputdim, bottleneck);
        let learning_rates = [0.001, 0.0001, 0.01];
        let filenames = [
            "encoded.model_0.001",
            "encoded.model_0.0001",
            "encoded.model_0.01",
        ];
        let mse_loss = |pred: &Tensor, target: &Tensor| pred.mse_loss(target, tch::Reduction::Mean);
        for (lr, _filename) in learning_rates.iter().zip(filenames.iter()) {
            println!("Training with learning rate: {}", lr);
            let mut opt = nn::Adam::default().build(&vs, *lr).unwrap();
            for epoch in 1..=epochs {
                opt.zero_grad();
                let output = autoencoder.forward(&traindata);
                let loss = mse_loss(&output, &traindata);
                let lossvalue = loss.to_kind(tch::Kind::Float).double_value(&[]);
                loss.backward();
                opt.step();
                if epoch % 10 == 0 || epoch == 1 {
                    println!("Epoch [{}/{}/{}]", epoch, epochs, lossvalue);
                }
            }
        }
        Ok("The autoencoder has finished".to_string())
    }
}

/*
 A multifunctional tensor from the multifasta file.Stacking all the tensors
*/
fn encode(seq: &str) -> Tensor {
    let dnaunpack = read_fasta(seq).unwrap();
    let mut genomseq: Vec<String> = Vec::new();
    for (_val, seq) in dnaunpack.iter() {
        genomseq.push(seq.seq.clone());
    }
    let mapping: HashMap<char, [f32; 4]> = [
        ('A', [1.0, 0.0, 0.0, 0.0]),
        ('T', [0.0, 1.0, 0.0, 0.0]),
        ('C', [0.0, 0.0, 1.0, 0.0]),
        ('G', [0.0, 0.0, 0.0, 1.0]),
    ]
    .iter()
    .cloned()
    .collect();
    let mut newvecstack: Vec<Vec<f32>> = Vec::new();
    for i in genomseq.iter() {
        let mut vecdata = vec![0f32; seq.len() * 4];
        for (i, c) in i.chars().enumerate() {
            let start = i * 4;
            if let Some(value) = mapping.get(&c) {
                vecdata[start..start + 4].copy_from_slice(value);
            }
        }
        newvecstack.push(vecdata);
    }
    let mut finaltensor = vec![];
    for i in newvecstack.iter() {
        let tensorvalue = Tensor::from_slice(&i).view([seq.len() as i64, 4]);
        finaltensor.push(tensorvalue);
    }
    let finaltensor = Tensor::stack(&finaltensor, 0);
    finaltensor
}
