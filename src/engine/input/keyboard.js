var keycode = 0;

document.addEventListener('keydown', function(event) {
    keycode = event.keyCode;
}, true);

document.addEventListener('keyup', function(event) {
    keycode = 0;
}, true);

export function getkeycode(){
    return keycode;
}