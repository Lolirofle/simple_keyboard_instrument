#![feature(type_ascription)]

extern crate cpal;
extern crate winit;

use std::collections::{HashMap,HashSet};
use std::f32;
use std::hash::{Hash,Hasher};
use std::sync;
use std::thread;

//Function which produces a sine wave (sinusoid)
fn sine(frequency: f32,amplitude: f32,sample_clock: f32,sample_rate: f32) -> f32{
	(sample_clock * frequency * f32::consts::PI / sample_rate).sin() * amplitude
}

//Function which produces a square wave
fn square(frequency: f32,amplitude: f32,sample_clock: f32,sample_rate: f32) -> f32{
	if (sample_clock * frequency) % (2.0 * sample_rate) <= sample_rate{
		amplitude
	}else{
		-amplitude
	}
}

//Function which produces a sawtooth wave
fn saw(frequency: f32,amplitude: f32,sample_clock: f32,sample_rate: f32) -> f32{
	(2.0 * ((frequency*sample_clock/sample_rate/2.0) % 1.0) - 1.0) * amplitude
}

//Function which produces a triangle wave
fn triangle(frequency: f32,amplitude: f32,sample_clock: f32,sample_rate: f32) -> f32{
	2.0 * saw(frequency,amplitude,sample_clock,sample_rate).abs() - 1.0
}

//Function which produces a constant (0)
fn const0(_frequency: f32,_amplitude: f32,_sample_clock: f32,_sample_rate: f32) -> f32{
	0.0
}

#[derive(Copy,Clone)]
struct F32Wrapper(f32);
impl PartialEq for F32Wrapper{
	fn eq(&self,other: &F32Wrapper) -> bool{
		self.0 == other.0
	}
}
impl Eq for F32Wrapper{}
impl Hash for F32Wrapper{
	fn hash<H: Hasher>(&self,state: &mut H){
		(self.0 as u32).hash(state);
	}
}

fn main(){
	let input = sync::Arc::new(sync::Mutex::new(HashMap::<F32Wrapper,(f32,&fn(f32,f32,f32,f32) -> f32)>::new()));
	let input2 = input.clone();

	let mut prev_freq: f32 = 0.0;
	thread::spawn(move ||{
		//Initialize audio
		let device = cpal::default_output_device().expect("Failed to get default output device");
		let format = device.default_output_format().expect("Failed to get default output format");
		let event_loop = cpal::EventLoop::new();
		let stream_id = event_loop.build_output_stream(&device,&format).unwrap();
		event_loop.play_stream(stream_id.clone());

		let sample_rate = format.sample_rate.0 as f32;
		let mut sample_clock = 0f32;
		event_loop.run(move |_,data|{
			match data{
				cpal::StreamData::Output{ buffer } => {
					let mut sound_fn = |sample_clock: f32,sample_rate: f32|{
						match input2.try_lock(){
							Ok(input3) => {
								prev_freq = input3.iter().map(move |(&frequency,&(amplitude,function))| function(frequency.0,amplitude,sample_clock,sample_rate)).sum();
								prev_freq
							},
							Err(_) => prev_freq,
						}
					};

					match buffer{
						cpal::UnknownTypeOutputBuffer::U16(mut buffer) => {
							for sample in buffer.chunks_mut(format.channels as usize){
								sample_clock = (sample_clock + 1.0) % sample_rate;
								let value = ((sound_fn(sample_clock,sample_rate) * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
								for out in sample.iter_mut(){
									*out = value;
								}
							}
						},
						cpal::UnknownTypeOutputBuffer::I16(mut buffer) => {
							for sample in buffer.chunks_mut(format.channels as usize){
								sample_clock = (sample_clock + 1.0) % sample_rate;
								let value = (sound_fn(sample_clock,sample_rate) * std::i16::MAX as f32) as i16;
								for out in sample.iter_mut(){
									*out = value;
								}
							}
						},
						cpal::UnknownTypeOutputBuffer::F32(mut buffer) => {
							for sample in buffer.chunks_mut(format.channels as usize){
								sample_clock = (sample_clock + 1.0) % sample_rate;
								let value = sound_fn(sample_clock,sample_rate);
								for out in sample.iter_mut(){
									*out = value;
								}
							}
						},
					}
				},
				_ => (),
			}
		});
	});

	let mut mappings = HashMap::new();
	{
		use winit::VirtualKeyCode::*;
		mappings.insert(Z,220.0);
		mappings.insert(X,220.0*(2.0: f32).powf(1.0/12.0));
		mappings.insert(C,220.0*(2.0: f32).powf(2.0/12.0));
		mappings.insert(V,220.0*(2.0: f32).powf(3.0/12.0));
		mappings.insert(B,220.0*(2.0: f32).powf(4.0/12.0));
		mappings.insert(N,220.0*(2.0: f32).powf(5.0/12.0));
		mappings.insert(M,220.0*(2.0: f32).powf(6.0/12.0));

		mappings.insert(A,440.0);
		mappings.insert(S,440.0*(2.0: f32).powf(1.0/12.0));
		mappings.insert(D,440.0*(2.0: f32).powf(2.0/12.0));
		mappings.insert(F,440.0*(2.0: f32).powf(3.0/12.0));
		mappings.insert(G,440.0*(2.0: f32).powf(4.0/12.0));
		mappings.insert(H,440.0*(2.0: f32).powf(5.0/12.0));
		mappings.insert(J,440.0*(2.0: f32).powf(6.0/12.0));
		mappings.insert(K,440.0*(2.0: f32).powf(7.0/12.0));
		mappings.insert(L,440.0*(2.0: f32).powf(8.0/12.0));

		mappings.insert(Q,880.0);
		mappings.insert(W,880.0*(2.0: f32).powf(1.0/12.0));
		mappings.insert(E,880.0*(2.0: f32).powf(2.0/12.0));
		mappings.insert(R,880.0*(2.0: f32).powf(3.0/12.0));
		mappings.insert(T,880.0*(2.0: f32).powf(4.0/12.0));
		mappings.insert(Y,880.0*(2.0: f32).powf(5.0/12.0));
		mappings.insert(U,880.0*(2.0: f32).powf(6.0/12.0));
		mappings.insert(I,880.0*(2.0: f32).powf(7.0/12.0));
		mappings.insert(O,880.0*(2.0: f32).powf(8.0/12.0));
		mappings.insert(P,880.0*(2.0: f32).powf(9.0/12.0));
	}

	let mut events_loop = winit::EventsLoop::new();

	let _window = winit::WindowBuilder::new()
		.with_title("Keyboard Piano")
		.build(&events_loop)
		.unwrap();

	let mut instrument: u8 = 0;
	let mut amplitude: f32 = 0.5;
	events_loop.run_forever(|event|{
		use winit::WindowEvent::*;
		use winit::ElementState::{Pressed,Released};

		match event{
			winit::Event::WindowEvent{ event, .. } => {
				use winit::VirtualKeyCode::*;
				match event{
					CloseRequested => {return winit::ControlFlow::Break;},
					KeyboardInput{input: winit::KeyboardInput{virtual_keycode: Some(virtual_code),state: Pressed,..},..} => match virtual_code{
						Left   => {if instrument > 0 {instrument-= 1;}},
						Right  => {if instrument < 3 {instrument+= 1;}},
						Up     => {if amplitude > 0.0 {amplitude-= 0.05;}},
						Down   => {if amplitude < 1.0 {amplitude+= 0.05;}},
						Escape => {return winit::ControlFlow::Break;},
						key => {
							if let Some(freq) = mappings.get(&key){
								let function = match instrument{
									0 => &(sine     as fn(_,_,_,_)->_),
									1 => &(square   as fn(_,_,_,_)->_),
									2 => &(saw      as fn(_,_,_,_)->_),
									3 => &(triangle as fn(_,_,_,_)->_),
									_ => &(const0   as fn(_,_,_,_)->_)
								};
								input.lock().unwrap().insert(F32Wrapper(*freq),(amplitude,function));
							}
						},
					},
					KeyboardInput{input: winit::KeyboardInput{virtual_keycode: Some(virtual_code),state: Released,..},..} => match virtual_code{
						key => {
							if let Some(freq) = mappings.get(&key){
								input.lock().unwrap().remove(&F32Wrapper(*freq));
							}
						},
					},
					_ => (),
				}
			},
			_ => (),
		}

		winit::ControlFlow::Continue
	});
}
