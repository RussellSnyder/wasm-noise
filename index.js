function createEventHandles(play_id, stop_id, rust_function, noiseType) {
    let handle = null;
 
    const play_button = document.getElementById(play_id);
    play_button.addEventListener("click", () => {
        handle = rust_function(noiseType);
    });
 
    const stop_button = document.getElementById(stop_id);
    stop_button.addEventListener("click", event => {
        if (handle != null) {
            handle.free();
	        handle = null;
        }
    });
}

import("./pkg").catch(console.error).then(rust_module=>{
    // createEventHandles('play-beep', 'stop-beep', rust_module.beep);
    createEventHandles('play-white-noise', 'stop-white-noise', rust_module.play_noise, 'White');
    createEventHandles('play-pink-noise', 'stop-pink-noise', rust_module.play_noise, 'Pink');
    // let beep_handle = null;
    // const play_beep_button = document.getElementById("play-beep");
    // play_beep_button.addEventListener("click", event => {
    //     beep_handle = rust_module.beep();
    // });
    // const stop_beep_button = document.getElementById("stop-beep");
    // stop_beep_button.addEventListener("click", event => {
    //     if (beep_handle != null) {
    //         beep_handle.free();
	//         beep_handle = null;
    //     }
    // });

    // let white_noise_handle = null;
    // const play_white_noise_button = document.getElementById("play-white-noise");
    // play_white_noise_button.addEventListener("click", event => {
    //     white_noise_handle = rust_module.white_noise();
    // });
    // const stop_white_noise_button = document.getElementById("stop-white-noise");
    // stop_white_noise_button.addEventListener("click", event => {
    //     if (white_noise_handle != null) {
    //         white_noise_handle.free();
	//         white_noise_handle = null;
    //     }
    // });
});
