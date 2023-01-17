// import * as wasm from "hello-wasm-pack";
import * as sim from "lib-simulation-wasm";
// import Chart from 'chart.js';
// alert("Who's that dog?" + sim.whos_that_dog() + "!");
const simulation = new sim.Simulation();
// document.getElementById("viewport").setAttribute("content", "width=1024, initial-scale=0, maximum-scale=1.0, minimum-scale=0.25, user-scalable=yes");

/** @type {HTMLCanvasElement} */
const viewport = document.getElementById("viewport");


const ctxt = viewport.getContext('2d');
const viewportWidth = viewport.width;
const viewportHight = viewport.height;
let minimum_acc = []
let average_acc = []
let maximum_acc = []
let labels = []
let generation = 1;

CanvasRenderingContext2D.prototype.drawCircle =
    function (x, y, radius) {
        ctxt.fillStyle = 'rgb(116, 211, 174)';

        this.beginPath();
        this.arc(x, y, radius, 0, 2.0 * Math.PI);
        this.fill();

    };

// function parse(stats) {
//     // let values = stats
// }

let ctx = document.getElementById('myChart').getContext('2d');
let chart = new Chart(ctx, {
    type: 'line',

    data: {
        labels: labels,
        datasets: [{
            label: 'Minimum',
            data: minimum_acc,
            backgroundColor: 'rgba(255, 99, 132, 0.2)',
            borderColor: 'rgba(255, 99, 132, 1)',
            borderWidth: 2
        },
        {
            label: 'Maximum',
            data: maximum_acc,
            backgroundColor: 'rgba(54, 162, 235, 0.2)',
            borderColor: 'rgba(54, 162, 235, 1)',
            borderWidth: 2
        },
        {
            label: 'Average',
            data: average_acc,
            backgroundColor: 'rgba(255, 206, 86, 0.2)',
            borderColor: 'rgba(255, 206, 86, 1)',
            borderWidth: 2
        }]
    },
    responsive: true,
    maintainAspectRatio: false,
    scales: {
        yAxes: [{
            ticks: {
                beginAtZero: true
            }
        }]
    }
});
document.getElementById("gener").innerText = generation;
let statsistic = "Updating value in a while";
document.getElementById('train').onclick = function () {
    statsistic = simulation.train();
    let result = parse();
    generation += 10;
    document.getElementById("gener").innerText = generation
    document.getElementById('min').innerText = result[0];
    document.getElementById('max').innerText = result[1];
    document.getElementById('avg').innerText = result[2];
    chart.data.datasets[0].data = minimum_acc;
    chart.data.datasets[1].data = maximum_acc;
    chart.data.datasets[2].data = average_acc;
    labels.push(minimum_acc.length);
    chart.update();
}



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


let step_count = 0;

function redraw() {
    ctxt.clearRect(0, 0, viewportWidth, viewportHight);
    simulation.step();
    step_count += 1;
    if (step_count % 2500 == 0) {
        generation += 1;
        step_count = 0;
    }

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





function parse() {
    let values = statsistic.split(',');
    let min = Number(values[0].split('=')[1]);
    let max = Number(values[1].split('=')[1]);
    let avg = Number(values[2].split('=')[1]);
    minimum_acc.push(min);
    maximum_acc.push(max);
    average_acc.push(avg);


    return [
        min,
        max,
        avg
    ]
}
