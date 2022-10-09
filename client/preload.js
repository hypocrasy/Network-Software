const { contextBridge, ipcRenderer } = require('electron')

contextBridge.exposeInMainWorld('electronAPI',{
  delay: (ip) => ipcRenderer.invoke('awesome:delay', ip),
  speed: (ip) => ipcRenderer.invoke('awesome:speed', ip),
})
