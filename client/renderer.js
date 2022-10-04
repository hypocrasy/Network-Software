var Words;
var TalkWords;
var TalkSub;
var xhr;
var delay_button;
var speed_button;

function InputPress() {
    // alert(document.getElementById("talkwords").value)
}

function isValidIP(ip) {
    var reg = /^(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\.(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\.(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\.(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])$/
    return reg.test(ip);
}

function new_req(str) {
    str = '<div class="btalk"><span>' + str + '</span></div>'
    Words.innerHTML = Words.innerHTML + str;
}

function new_res(str) {
    str = '<div class="atalk"><span>' + str + '</span></div>';
    Words.innerHTML = Words.innerHTML + str;
}



async function delay_test(ip) {
    if (!isValidIP(ip)) {
        // new_res("尝试测试网络延迟... <br> 错误的 IP!");
        new_res("错误的 ip")
        return;
    }
    new_res("尝试测试网络延迟, 请稍后")

    let res = await window.electronAPI.delay(ip)
    if (res == -1) {
        new_res("不可达")
    } else {
        new_res(res + "ms")
    }
}

function speed_test(ip) {
    new_res("尚未实现")
    return

    // todo
    if (!isValidIP(ip)) {
        new_res("尝试测试网速... <br> 错误的 IP!");
    }
    // new_res("尝试测试网速, 请稍后")

}

function init() {
    Words = document.getElementById("words");
    TalkWords = document.getElementById("talkwords");
    delay_button = document.getElementById("delay_button")
    speed_button = document.getElementById("speed_button")

    delay_button.addEventListener('click', async () => {
        const filePath = await window.electronAPI.openFile()
        new_res(filePath)
    })
      
      


    delay_button.onclick = function () {
        delay_test(TalkWords.value)
    }
    speed_button.onclick = function () {
        speed_test(TalkWords.value)
    }
}

window.onload = init



// exports.init = init;