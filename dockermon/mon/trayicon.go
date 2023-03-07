package mon

import (
	"bytes"
	"fmt"
	"image"
	"image/color"
	"image/png"
	"io/ioutil"
	"math"
	"os"
	"os/exec"
	"path"
	"strings"
	"time"

	docker "github.com/fsouza/go-dockerclient"
	"github.com/getlantern/systray"
)

var command string
var currentItem *systray.MenuItem
var dockermonHome string
var icon []byte
var grayIcon []byte

func Start(args []string) error {
	homeDir, _ := os.UserHomeDir()
	dockermonHome = path.Join(homeDir, ".local/share/dockermon")
	if len(args) < 2 {
		command = path.Join(dockermonHome, "call-docker")
	} else {
		command = strings.Join(args[1:], " ")
	}
	systray.Run(onReady, func() {})
	return nil
}

func onReady() {
	loadIcon()
	loadGrayIcon()
	systray.SetIcon(icon)
	systray.SetTitle("0")
	systray.SetTooltip("loading...")
	currentItem = systray.AddMenuItem("loading...", command)
	systray.AddSeparator()
	quit := systray.AddMenuItem("⏻ Quit DockerMon", "quit")
	go update()
	for {
		select {
		case <-quit.ClickedCh:
			systray.Quit()
		case <-currentItem.ClickedCh:
			exec.Command(command).Run()
		}
	}
}

func update() {
	client, err := docker.NewClientFromEnv()
	if err != nil {
		panic(err)
	}

	for {
		if containers, err := client.ListContainers(docker.ListContainersOptions{}); err == nil {
			if count := len(containers); count == 0 {
				systray.SetIcon(grayIcon)
				systray.SetTitle("0")
				systray.SetTooltip("No running containers")
				currentItem.SetTitle("No running containers")
			} else {
				title := make([]string, 0)
				for _, item := range containers {
					title = append(title, strings.Join(item.Names, ", "))
				}
				systray.SetIcon(icon)
				systray.SetTitle(fmt.Sprintf("%d", count))
				if count == 1 {
					systray.SetTooltip("1 running container")
				} else {
					systray.SetTooltip(fmt.Sprintf("%d running containers", count))
				}
				currentItem.SetTitle(strings.Join(title, "\n"))
			}
		} else {
			systray.SetTitle("‼️")
			systray.SetTooltip(err.Error())
			currentItem.SetTitle(err.Error())
		}
		time.Sleep(5 * time.Second)
	}
}

func loadIcon() {
	iconPath := path.Join(dockermonHome, "/docker.png")
	data, err := ioutil.ReadFile(iconPath)
	if err != nil {
		panic(err)
	}
	icon = data
}

func loadGrayIcon() {
	img, _, err := image.Decode(bytes.NewReader(icon))
	if err != nil {
		grayIcon = icon
		return
	}
	size := img.Bounds().Max
	grayImg := image.NewRGBA(image.Rectangle{image.Point{0, 0}, size})
	for y := 0; y < size.Y; y++ {
		for x := 0; x < size.X; x++ {
			clr := img.At(x, y)
			rr, gg, bb, a := clr.RGBA()
			r := math.Pow(float64(rr), 2.2)
			g := math.Pow(float64(gg), 2.2)
			b := math.Pow(float64(bb), 2.2)
			mm := math.Pow(0.2125*r+0.7154*g+0.0721*b, 1.0/2.2)
			m := uint8(uint16(mm+0.5) >> 8)
			grayImg.Set(x, y, color.RGBA{m, m, m, uint8(a)})
		}
	}
	buffer := bytes.NewBuffer([]byte{})
	png.Encode(buffer, grayImg)
	grayIcon = buffer.Bytes()
}
