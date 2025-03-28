
export function connect(target) {
    return new Promise((resolve, reject) => {
        let ws = new WebSocket(target);
        ws.addEventListener("open", function() {
            resolve(new AsyncWS(ws));
        });
        ws.addEventListener("error", function(e) {
            reject(e);
        });
    });
}

export class AsyncWS {
    constructor(ws) {
        // Why doesn't javascript have a ringbuffer, or any efficient FIFO structure...
        this.waiters = [];
        this.messages = [];

        this.ws = ws;
        this.ws.addEventListener("close", () => {
            this.closed = true;
            for (let [_, reject] of this.waiters) {
                reject("connection closed");
            }
            this.waiters.length = 0;
        });
        this.ws.addEventListener("error", (e) => {
            this.closed = true;
            for (let [_, reject] of this.waiters) {
                reject(e);
            }
            this.waiters.length = 0;
        });
        this.ws.addEventListener("message", (message) => {
            let waiter = this.waiters.shift();
            if (waiter != null) {
                waiter[0](message.data);
            } else {
                this.messages.push(message.data);
            }
        });
        this.closed = ws.readyState == ws.CLOSING || ws.readyState == ws.CLOSED;
    }
    send(data) {
        if (this.closed) {
            throw new Error("socket closed");
        }
        this.ws.send(data);
    }
    recv() {
        return new Promise((resolve, reject) => {
            let x = this.messages.shift();
            if (x != null) {
                resolve(x);
            } else if (this.closed) {
                reject("channel is closed");
            } else {
                this.waiters.push([resolve, reject]);
            }
        }).catch((error) => {
            console.warn("Error receiving from channel:", error);
            return null;
        });
    }
    close() {
        this.ws.close();
    }
}
