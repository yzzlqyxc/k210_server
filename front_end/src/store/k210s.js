import $ from 'jquery'

export const ModuleK210 = {
    state () {
        return {
            loadedK210s : [],
            history : [],
            remotePort : ""
        }
    },
    getters : {

    },
    mutations : {
        updateLoadedK210s(state, ports) {
            const t = JSON.parse(ports)
            console.log(t);
            state.loadedK210s = t.addrs
        },
        updatePort(state, port) {
            state.remotePort = port
        }, 
        updateHistories(state, history) {
            const t = JSON.parse(history)
            console.log(t.histories);
            state.history = t.histories
        }
    },
    actions : {
        uploadK210s(context) {
            $.ajax({
                url: "http://47.93.124.97:3000/getUserList",
                type: "GET",
                data: {

                },
                success(resp) {
                    context.commit("updateLoadedK210s", resp);
                    // console.log(resp);
                },
                error(resp) {
                    console.log(resp)
                }
            })
        },
        uploadHistories(context) {
            if (context.state.remotePort == "") {
                return 
            }
            $.ajax({
                url: `http://47.93.124.97:3000/getUserHistory/${context.state.remotePort}`,
                type : "GET",
                success(resp) {
                    context.commit("updateHistories", resp)
                },
                error(resp) {
                    console.log(resp);
                }
            })
        }
    }
}