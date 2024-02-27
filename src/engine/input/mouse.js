var px = 0;
var py = 0;

document.addEventListener("mousemove", function(event){
    px = event.movementX;
    py = event.movementY;
}, false);     

export function jgetx(){
    var t = px;
    px = 0;
    return Number(t); 
}

export function jgety(){
    var t = py;
    py = 0;
    return Number(t); 
}