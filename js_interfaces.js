mergeInto(LibraryManager.library, {
  hide_loading: function(num) {
    var loading = document.getElementById('loading')
    loading.parentNode.removeChild(loading)
  },
  play_audio: function(num) {
    var audio = []
    for (var i = 0; i < audio.length; i++) {
      if (i === num) {
        audio[i].play()
      } else {
        audio[i].pause()
        audio[i].currentTime = 0
      }
    }
  },
})
