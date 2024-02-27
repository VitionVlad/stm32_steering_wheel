var tx = 0;
var ty = 0;
var inuse = false;
var index = 0;

document.addEventListener("touchmove", function(event){
    tx = event.touches[index].clientX;
    ty = event.touches[index].clientY;
}, false);     

document.addEventListener("touchend", function(){
    inuse = false;
}, false); 

document.addEventListener("touchstart", function(){
    inuse = true;
}, false); 

export function jgettx(){
    return Number(tx); 
}

export function jgetty(){
    return Number(ty); 
}

export function jgetuse(){
    return Number(inuse); 
}

export function jsettouchindex(lindex){
    index = lindex;
}