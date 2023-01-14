// import * as wasm from "hello-wasm-pack";
import * as sim from "lib-simulation-wasm";
// alert("Who's that dog?" + sim.whos_that_dog() + "!");
const simulation = new sim.Simulation();
// document.getElementById("viewport").setAttribute("content", "width=1024, initial-scale=0, maximum-scale=1.0, minimum-scale=0.25, user-scalable=yes");

/** @type {HTMLCanvasElement} */
const viewport = document.getElementById("viewport");


const ctxt = viewport.getContext('2d');
const viewportWidth = viewport.width;
const viewportHight = viewport.height;
CanvasRenderingContext2D.prototype.drawCircle =
function (x, y, radius) {
        ctxt.fillStyle = 'rgb(116, 211, 174)';

        this.beginPath();
        this.arc(x, y, radius, 0, 2.0 * Math.PI);
        
        
        // this.fillStyle = 'rgb(0, 0, 0)';
        this.fill();

    };

CanvasRenderingContext2D.prototype.drawTriangle = function (x, y, size, rotation) {
    this.beginPath();
    this.fillStyle = 'rgb(234, 190, 124)';
    this.moveTo(
        x - Math.sin(rotation) * size,
        y + Math.cos(rotation) * size,
        );
        
        this.lineTo(
            x - Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size,
            y + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size,
            );
            
            this.lineTo(
                x - Math.sin(rotation + 4.0 / 3.0 * Math.PI) * size,
                y + Math.cos(rotation + 4.0 / 3.0 * Math.PI) * size,
                );
                
                this.lineTo(
                    x - Math.sin(rotation) * size,
                    y + Math.cos(rotation) * size,
                    );
                    
                    this.stroke();
                    this.fill();
};


function redraw() {
    ctxt.clearRect(0, 0, viewportWidth, viewportHight);
    simulation.step();

    for (const food of simulation.world().foods) {
        ctxt.drawCircle(
            food.x * viewportWidth,
            food.y * viewportHight,
            (0.01 / 2.0) * viewportWidth
        )
    }

    for (const animal of simulation.world().animals) {
        // console.log(animal)
        ctxt.drawTriangle(
            animal.x * viewportWidth,
            animal.y * viewportHight,
            0.01 * viewportWidth,
            animal.rotation
        );
    }
    requestAnimationFrame(redraw);
}
redraw();