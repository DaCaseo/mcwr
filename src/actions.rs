pub fn start_server(args: &super::Args, mut tmux: tmux_interface::TmuxInterface) {
	let new_session = tmux_interface::NewSession {
		cwd: Some("./server"),
		session_name: Some(&args.name),
		shell_command: Some("java -Xms1G -Xmx3G -jar ./server.jar nogui"),
		..Default::default()
	};
	tmux.new_session(Some(&new_session)).expect("failed to create session");
}

pub fn stop_server(args: &super::Args, mut tmux: tmux_interface::TmuxInterface) {
	 let stop_session = tmux_interface::SendKeys {
		 target_pane: Some(&args.name),
		 ..Default::default()
	 };
	 tmux.send_keys(Some(&stop_session), &vec!("stop", "Enter")).expect("failed to send stop");
}
