//
// interface.go
//
// Interface example using media players as an example
package main

import "fmt"

// Example interface for a media player.  All media types whether audio or video
// usually have the same actions
type mediaPlayer interface {
	play()
	pause()
	prev()
	next()
	PrintDescription()
}

// Audio player that satisfies the media type interface.  Note that all we need
// to do to satisfy the interface is to declare all the functions.  We will at
// that point satisfy the interface implicitly
type audioPlayer struct {
	description string
}

func (ap audioPlayer) PrintDescription() {
	fmt.Println("I'm a " + ap.description + "player")
}

func (ap audioPlayer) play() {
	fmt.Println("Playing a song")
}

func (ap audioPlayer) pause() {
	fmt.Println("Pausing a song")
}

func (ap audioPlayer) prev() {
	fmt.Println("Previous song")
}

func (ap audioPlayer) next() {
	fmt.Println("Next song")
}

// Define a video player type using the mediaPlayer interface that is slightly
// different from the audio player
type videoPlayer struct{}

func (vp videoPlayer) PrintDescription() {
	fmt.Println("I'm a video player")
}

func (vp videoPlayer) play() {
	fmt.Println("Playing a video")
}

func (vp videoPlayer) pause() {
	fmt.Println("Pausing a video")
}

func (vp videoPlayer) prev() {
	fmt.Println("Previous video")
}

func (vp videoPlayer) next() {
	fmt.Println("Next video")
}

func exerciseMediaPlayer(mp mediaPlayer) {
	mp.PrintDescription()
	mp.prev()
	mp.play()
	mp.next()
	mp.play()
	mp.pause()
}

func main() {
	var ap audioPlayer
	var vp videoPlayer
	ap.description = "audio"
	exerciseMediaPlayer(ap)
	exerciseMediaPlayer(vp)
}
