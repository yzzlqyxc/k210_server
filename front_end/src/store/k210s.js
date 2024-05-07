import $ from 'jquery'

export const ModuleK210 = {
    state () {
        return {
            loadedK210s : ["fdasf"],
            key : 1
        }
    },
    getters : {

    },
    mutations : {
        updateLoadedK210s(state, ports) {
            state.loadedK210s.push("fdas")
            console.log(state.loadedK210s, ports);
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

        }
        
    }
}