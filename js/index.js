// import("../pkg/index.js").catch(console.error);
(async () => {
    const wasm = await import("../pkg/index.js").catch(console.error);
    const wasm_bg = await import("../pkg/index_bg.wasm").catch(console.error);

    wasm_bg.init(50, 50);
    let handler = {};
    let bef = 0;
    let draw = true;
    let cfps = 1;
    let fps = 1;

    let loop = (t) => {
        if (fps > 0 && (t - bef >= 1000 / fps || t < bef)) {
            bef = t;
            wasm_bg.step();
            draw = true;
        }
        if (draw)
            wasm_bg.draw();
        draw = false;
        handler.id = requestAnimationFrame(loop);
    }
    handler.id = requestAnimationFrame(loop);
    // ループをキャンセル
    // cancelAnimationFrame(handler.id);

    let onClick = (e) => {
        var rect = e.target.getBoundingClientRect();
        let x = e.clientX - rect.left;
        let y = e.clientY - rect.top;
        wasm_bg.click(x, y);
        draw = true;
        // console.log("(" + x + ", " + y + ")");
    }

    let slider = document.getElementById('slider');
    let svalue = document.getElementById('slider-value');

    let setFPS = (f) => {
        svalue.innerText = f;
        fps = f;
    };

    slider.addEventListener(
        'change', (e) => setFPS(e.target.value), false
    );

    document.getElementById('canvas').addEventListener(
        'click', onClick, false
    );
    document.getElementById('button_auto').addEventListener(
        'click', () => { setFPS(cfps); }, false
    );
    document.getElementById('button_stop').addEventListener(
        'click', () => { cfps = fps; setFPS(0); }, false
    );
    document.getElementById('button_step').addEventListener(
        'click', () => { wasm_bg.step(); draw = true; }, false
    );


})();