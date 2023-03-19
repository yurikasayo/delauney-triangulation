import './style.css';
import init, { App } from './pkg/wasm';

window.onload = _ => {
    init().then(_ => {
        let canvas = document.getElementById('webgl');
        let app = App.new(window, canvas);

        // canvas.addEventListener('mousedown', (e) => {
        //     app.mouse_down(e.x, e.y);
        // })

        // canvas.addEventListener('mousemove', (e) => {
        //     app.mouse_move(e.x, e.y);
        // })

        // canvas.addEventListener('mouseup', (e) => {
        //     app.mouse_up(e.x, e.y);
        // })

        // window.addEventListener('resize', _ => app.resize)

        app.render();
    });
}