const {app, BrowserWindow, ipcMain, dialog} = require('electron')
const path = require('path')
const dgram = require('dgram')
const {randomUUID} = require('crypto')
const { promisify } = require('util')
const sleep = promisify(setTimeout)
//? 更好的办法是使用事件触发器，但是我懒
const events = require('events')
const net = require('net')
const { EventEmitter } = require('stream')

var last_ping

async function handleFileOpen() {
  const { canceled, filePaths } = await dialog.showOpenDialog()
  if (canceled) {
    return
  } else {
    return filePaths[0]
  }
}

function createWindow () {
  const mainWindow = new BrowserWindow({
    webPreferences: {
      preload: path.join(__dirname, 'preload.js')
    }
  })
  mainWindow.loadFile('index.html')
}

function draw_chat() {
  const chartWindow = new BrowserWindow({
    webPreferences: {
      preload: path.join(__dirname, 'echarts_preload.js')
    }
  })
  chartWindow.loadFile('echart.html')
}

app.whenReady().then(() => {
  ipcMain.handle('dialog:openFile', handleFileOpen)
  ipcMain.handle('awesome:delay', async (event, ...args) => {
    console.log(args[0])
    let arr = new Array
    let sum = 0
    for(let i=0; i<7; i++) {
      let res = await ping(...args)
      sum += res
      arr.push(res)
    }
    // const result = await ping(...args)
    if(arr.includes(-1)) {
      return -1
    } else {
      last_ping = arr
      console.log(arr)
      // todo 没跑通, 原来的跨进程通信方式不适用于现在的窗口
      // draw_chat()
      return sum/7
    }
  })
  ipcMain.handle('awesome:speed', async (event, ...args) => {
    const result = await speed(...args)
    return result
  })

  ipcMain.handle('chart:last_ping', async (event, ...args) => {
    return last_ping
  })
  createWindow()
  app.on('activate', function () {
    if (BrowserWindow.getAllWindows().length === 0) createWindow()
  })
})

app.on('window-all-closed', function () {
  if (process.platform !== 'darwin') app.quit()
})


async function ping(ip) {
  let wl = new Map;
  let so = dgram.createSocket('udp4')
  so.on('message', (msg, _) => {
    //! may cause bug
    let res = msg.toString().substring(0, 36)
    // console.log(res)
    // console.log(encodeURIComponent(res))
    if (wl.has(res)) {
      wl.set(res, Date.now())
    } else {
      console.log("Fatal Error")
    }
  })

  let uuid = randomUUID()
  let t1 = Date.now();
  wl.set(uuid, 0)
  so.send(uuid, 34254, ip)
  let t2 = 0;
  for (let i = 0; i < 30; i++) {
    await sleep(100);
    if (wl.get(uuid) != 0) {
      t2 = wl.get(uuid)
      break
    }
  }
  if (t2 == 0) {
    return -1
  } else {
    return (t2-t1)
  }
}

async function speed(ip) {
  let arr = new Array()
  let client = new net.Socket
  let fatal = false
  client.on('error', () => {
    fatal = true
  })
  client.connect(34254, ip, () => {
    console.log("connect successful")
  
  })
  let flag = false
  client.on("data", (data) => {
    if(data.length >= 4 && data.slice(data.length-4, data.length).readInt32BE() == 1919810) {
      console.log("ok")
      flag = true
    } else {
      arr.push([data.length, Date.now()])
    }
  })
  for(let i=0;i<30 && !flag;i++) {
    await sleep(200)
  }
  if((flag == false && arr.length == 0) || fatal) {
    return -1
  } else {
    let consume = arr[arr.length-1][1] - arr[0][1]
    let sum = 0
    arr.forEach((val) => sum += val[0])
    console.log("consume", consume, "sum", sum)
    return sum/consume
  }
  // 可改成更高效的 eventemit 形式
  
}

// function timeout(fun, time) {
//   return (...args) => {
//     return Promise.race([fun(...args), new Promise((_, rej) => {
//       setTimeout(rej, time, new Error('超时了!'));
//     }),]);
//   };
// }