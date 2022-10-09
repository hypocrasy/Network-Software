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

app.whenReady().then(() => {
  ipcMain.handle('dialog:openFile', handleFileOpen)
  ipcMain.handle('awesome:delay', async (event, ...args) => {
    const result = await ping(...args)
    return result
  })
  ipcMain.handle('awesome:speed', async (event, ...args) => {
    const result = await speed(...args)
    return result
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
  console.log(ip)
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
  client.connect(34254, ip, () => {
    console.log("connect successful")
  
  })
  let flag = false
  client.on("data", (data) => {
    // console.log(data.length)
    if(data.readInt32BE() == 1919810) {
      console.log("ok")
      flag = true
    } else {
      arr.push([data.length, Date.now()])
    }
  })
  for(;!flag;) {
    await sleep(200)
  }
  let consume = arr[arr.length-1][1] - arr[0][1]
  let sum = 0
  arr.forEach((val) => sum += val[0])
  console.log("consume", consume, "sum", sum)
  return sum/consume
  // 可改成更高效的 eventemit 形式
  
  
  
  
}

// function timeout(fun, time) {
//   return (...args) => {
//     return Promise.race([fun(...args), new Promise((_, rej) => {
//       setTimeout(rej, time, new Error('超时了!'));
//     }),]);
//   };
// }