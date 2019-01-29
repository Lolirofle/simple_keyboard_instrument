#![feature(type_ascription)]

extern crate cpal;
extern crate winit;

use std::collections::{HashMap,HashSet};
use std::f64;
use std::hash::{Hash,Hasher};
use std::mem;
use std::sync;
use std::thread;

//Function which produces a sine wave (sinusoid)
fn sine(frequency: f64,amplitude: f64,sample_clock: f64,sample_rate: f64) -> f64{
	(sample_clock * frequency * f64::consts::PI / sample_rate).sin() * amplitude
}

//Function which produces a square wave
fn square(frequency: f64,amplitude: f64,sample_clock: f64,sample_rate: f64) -> f64{
	if (sample_clock * frequency) % (2.0 * sample_rate) <= sample_rate{
		amplitude
	}else{
		-amplitude
	}
}

//Function which produces a sawtooth wave
fn saw(frequency: f64,amplitude: f64,sample_clock: f64,sample_rate: f64) -> f64{
	(2.0 * ((frequency*sample_clock/sample_rate/2.0) % 1.0) - 1.0) * amplitude
}

//Function which produces a triangle wave
fn triangle(frequency: f64,amplitude: f64,sample_clock: f64,sample_rate: f64) -> f64{
	2.0 * saw(frequency,amplitude,sample_clock,sample_rate).abs() - 1.0
}

//Function which produces a constant (0)
fn const0(_frequency: f64,_amplitude: f64,_sample_clock: f64,_sample_rate: f64) -> f64{
	0.0
}

//f64 with Eq and Hash
#[derive(Copy,Clone)]
struct F64Wrapper(f64);
impl PartialEq for F64Wrapper{
	fn eq(&self,other: &F64Wrapper) -> bool{
		self.0 == other.0
	}
}
impl Eq for F64Wrapper{}
impl Hash for F64Wrapper{
	fn hash<H: Hasher>(&self,state: &mut H){
		unsafe{mem::transmute::<f64,u64>(self.0)}.hash(state);
	}
}

fn main(){
	//Map of all tones playing at the moment
	let input = sync::Arc::new(sync::Mutex::new(HashMap::<F64Wrapper,(f64,&fn(f64,f64,f64,f64) -> f64)>::new()));
	let input2 = input.clone();

	//Audio thread
	let mut prev_value: f64 = 0.0;
	thread::spawn(move ||{
		//Initialize audio
		let device = cpal::default_output_device().expect("Failed to get default output device");
		let format = device.default_output_format().expect("Failed to get default output format");
		let event_loop = cpal::EventLoop::new();
		let stream_id = event_loop.build_output_stream(&device,&format).unwrap();
		event_loop.play_stream(stream_id.clone());

		let sample_rate = format.sample_rate.0 as f64;
		let mut sample_clock = 0.0;
		event_loop.run(move |_,data|{
			match data{
				cpal::StreamData::Output{ buffer } => {
					let mut sound_fn = |sample_clock: f64,sample_rate: f64|{
						match input2.try_lock(){
							Ok(input3) => {
								prev_value = input3.iter().map(move |(&frequency,&(amplitude,function))| function(frequency.0,amplitude,sample_clock,sample_rate)).sum();
								prev_value
							},
							Err(_) => prev_value,
						}
					};

					match buffer{
						cpal::UnknownTypeOutputBuffer::U16(mut buffer) => {
							for sample in buffer.chunks_mut(format.channels as usize){
								sample_clock = (sample_clock + 1.0) % sample_rate;
								let value = ((sound_fn(sample_clock,sample_rate) * 0.5 + 0.5) * std::u16::MAX as f64) as u16;
								for out in sample.iter_mut(){ //TODO: Not sure why the whole buffer is filled with the same value
									*out = value;
								}
							}
						},
						cpal::UnknownTypeOutputBuffer::I16(mut buffer) => {
							for sample in buffer.chunks_mut(format.channels as usize){
								sample_clock = (sample_clock + 1.0) % sample_rate;
								let value = (sound_fn(sample_clock,sample_rate) * std::i16::MAX as f64) as i16;
								for out in sample.iter_mut(){
									*out = value;
								}
							}
						},
						cpal::UnknownTypeOutputBuffer::F64(mut buffer) => {
							for sample in buffer.chunks_mut(format.channels as usize){
								sample_clock = (sample_clock + 1.0) % sample_rate;
								let value = sound_fn(sample_clock,sample_rate) as f32;
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

	//Key mappings. Which keys that trigger certain sounds
	let mut mappings = HashMap::<winit::ScanCode,f64>::new();
	{
		mappings.insert(41,1760.0);
		mappings.insert(2 ,1760.0 * (2.0: f64).powf(1.0/12.0));
		mappings.insert(3 ,1760.0 * (2.0: f64).powf(2.0/12.0));
		mappings.insert(4 ,1760.0 * (2.0: f64).powf(3.0/12.0));
		mappings.insert(5 ,1760.0 * (2.0: f64).powf(4.0/12.0));
		mappings.insert(6 ,1760.0 * (2.0: f64).powf(5.0/12.0));
		mappings.insert(7 ,1760.0 * (2.0: f64).powf(6.0/12.0));
		mappings.insert(8 ,1760.0 * (2.0: f64).powf(7.0/12.0));
		mappings.insert(9 ,1760.0 * (2.0: f64).powf(8.0/12.0));
		mappings.insert(10,1760.0 * (2.0: f64).powf(9.0/12.0));
		mappings.insert(11,1760.0 * (2.0: f64).powf(10.0/12.0));
		mappings.insert(12,1760.0 * (2.0: f64).powf(11.0/12.0));

		mappings.insert(16,880.0);
		mappings.insert(17,880.0 * (2.0: f64).powf(1.0/12.0));
		mappings.insert(18,880.0 * (2.0: f64).powf(2.0/12.0));
		mappings.insert(19,880.0 * (2.0: f64).powf(3.0/12.0));
		mappings.insert(20,880.0 * (2.0: f64).powf(4.0/12.0));
		mappings.insert(21,880.0 * (2.0: f64).powf(5.0/12.0));
		mappings.insert(22,880.0 * (2.0: f64).powf(6.0/12.0));
		mappings.insert(23,880.0 * (2.0: f64).powf(7.0/12.0));
		mappings.insert(24,880.0 * (2.0: f64).powf(8.0/12.0));
		mappings.insert(25,880.0 * (2.0: f64).powf(9.0/12.0));
		mappings.insert(26,880.0 * (2.0: f64).powf(10.0/12.0));

		mappings.insert(30,440.0 ) ;
		mappings.insert(31,440.0 * (2.0: f64).powf(1.0/12.0));
		mappings.insert(32,440.0 * (2.0: f64).powf(2.0/12.0));
		mappings.insert(33,440.0 * (2.0: f64).powf(3.0/12.0));
		mappings.insert(34,440.0 * (2.0: f64).powf(4.0/12.0));
		mappings.insert(35,440.0 * (2.0: f64).powf(5.0/12.0));
		mappings.insert(36,440.0 * (2.0: f64).powf(6.0/12.0));
		mappings.insert(37,440.0 * (2.0: f64).powf(7.0/12.0));
		mappings.insert(38,440.0 * (2.0: f64).powf(8.0/12.0));
		mappings.insert(39,440.0 * (2.0: f64).powf(9.0/12.0));
		mappings.insert(40,440.0 * (2.0: f64).powf(10.0/12.0));
		mappings.insert(43,440.0 * (2.0: f64).powf(11.0/12.0));

		mappings.insert(86,220.0 ) ;
		mappings.insert(44,220.0 * (2.0: f64).powf(1.0/12.0));
		mappings.insert(45,220.0 * (2.0: f64).powf(2.0/12.0));
		mappings.insert(46,220.0 * (2.0: f64).powf(3.0/12.0));
		mappings.insert(47,220.0 * (2.0: f64).powf(4.0/12.0));
		mappings.insert(48,220.0 * (2.0: f64).powf(5.0/12.0));
		mappings.insert(49,220.0 * (2.0: f64).powf(6.0/12.0));
		mappings.insert(50,220.0 * (2.0: f64).powf(7.0/12.0));
		mappings.insert(51,220.0 * (2.0: f64).powf(8.0/12.0));
		mappings.insert(52,220.0 * (2.0: f64).powf(9.0/12.0));
		mappings.insert(53,220.0 * (2.0: f64).powf(10.0/12.0));
	}

	//Initialize window
	let mut events_loop = winit::EventsLoop::new();
	let _window = winit::WindowBuilder::new()
		.with_title("Keyboard Piano")
		.build(&events_loop)
		.unwrap();

	let mut instrument: u8 = 0;
	let mut amplitude: f64 = 0.5;
	events_loop.run_forever(|event|{
		use winit::WindowEvent::*;
		use winit::ElementState::{Pressed,Released};

		match event{
			winit::Event::WindowEvent{ event, .. } => {
				use winit::VirtualKeyCode::*;
				match event{
					//Exit
					CloseRequested => {return winit::ControlFlow::Break;},
					KeyboardInput{input: winit::KeyboardInput{virtual_keycode: Some(Escape),state: Pressed,..},..} => {return winit::ControlFlow::Break;},

					//Change instrument
					KeyboardInput{input: winit::KeyboardInput{virtual_keycode: Some(Left)  ,state: Pressed,..},..} => {if instrument > 0 {instrument-= 1;}},
					KeyboardInput{input: winit::KeyboardInput{virtual_keycode: Some(Right) ,state: Pressed,..},..} => {if instrument < 3 {instrument+= 1;}},

					//Change volume
					KeyboardInput{input: winit::KeyboardInput{virtual_keycode: Some(Up)    ,state: Pressed,..},..} => {if amplitude > 0.0 {amplitude-= 0.05;}},
					KeyboardInput{input: winit::KeyboardInput{virtual_keycode: Some(Down)  ,state: Pressed,..},..} => {if amplitude < 1.0 {amplitude+= 0.05;}},

					//Tone keys
					KeyboardInput{input: winit::KeyboardInput{scancode,state: Pressed,..},..} => {
						if let Some(freq) = mappings.get(&scancode){
							let function = match instrument{
								0 => &(sine     as fn(_,_,_,_)->_),
								1 => &(square   as fn(_,_,_,_)->_),
								2 => &(saw      as fn(_,_,_,_)->_),
								3 => &(triangle as fn(_,_,_,_)->_),
								_ => &(const0   as fn(_,_,_,_)->_)
							};
							input.lock().unwrap().insert(F64Wrapper(*freq),(amplitude,function));
						}
					},
					KeyboardInput{input: winit::KeyboardInput{scancode,state: Released,..},..} => {
						if let Some(freq) = mappings.get(&scancode){
							input.lock().unwrap().remove(&F64Wrapper(*freq));
						}
					},

					_ => (),
				}
			},
			_ => (),
		}

		winit::ControlFlow::Continue
	});
}
