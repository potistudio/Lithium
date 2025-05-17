use winit::application::ApplicationHandler;
use winit::event::{ WindowEvent, DeviceEvent, DeviceId };
use winit::event_loop::{ EventLoop, ActiveEventLoop };
use winit::window::{ Window, WindowId };

#[derive(Default)]
struct State {
	window: Option<Window>,
	counter: i32,
}

impl ApplicationHandler for State {
	//* This is a common indicator that you can create a window.
	fn resumed (&mut self, event_loop: &ActiveEventLoop) {
		self.window = Some (event_loop.create_window(Window::default_attributes()).unwrap());
	}


	fn window_event (&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
		// `unwrap` is fine, the window will always be available when receiving a window event.
		let window = self.window.as_ref().unwrap();

		//* Handle window event. *//
	}


	fn device_event (&mut self, event_loop: &ActiveEventLoop, device_id: DeviceId, event: DeviceEvent) {
		//* Handle window event. *//
	}


	fn about_to_wait (&mut self, event_loop: &ActiveEventLoop) {
		if let Some (window) = self.window.as_ref() {
			window.request_redraw();
			self.counter += 1;
		}
	}
}

fn main() {
	let event_loop = EventLoop::new().unwrap();
	let mut state = State::default();
	let _ = event_loop.run_app (&mut state);
}
