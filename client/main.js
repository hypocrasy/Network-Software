const {app, BrowserWindow, ipcMain, dialog} = require('electron')
const path = require('path')
const dgram = require('dgram')
const {randomUUID} = require('crypto')
const { promisify } = require('util')
const sleep = promisify(setTimeout)
//? 更好的办法是使用事件触发器，但是我懒
const events = require('events')

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
  createWindow()
  app.on('activate', function () {
    if (BrowserWindow.getAllWindows().length === 0) createWindow()
  })
})

app.on('window-all-closed', function () {
  if (process.platform !== 'darwin') app.quit()
})


async function ping(ip) {
  events.EventEmitter
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

// function timeout(fun, time) {
//   return (...args) => {
//     return Promise.race([fun(...args), new Promise((_, rej) => {
//       setTimeout(rej, time, new Error('超时了!'));
//     }),]);
//   };
// }