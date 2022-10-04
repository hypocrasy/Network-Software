const { contextBridge, ipcRenderer } = require('electron')

contextBridge.exposeInMainWorld('electronAPI',{
  delay: (ip) => ipcRenderer.invoke('awesome:delay', ip),
})
