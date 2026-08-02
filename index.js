import init, { distance } from "./pkg/pylev.js";

async function start() {
    await init();

    window.calc = function () {
        const a = document.getElementById("a").value;
        const b = document.getElementById("b").value;

        const d = distance(a, b);

        document.getElementById("result").innerText =
            "Distance: " + d;
    };
}

start();