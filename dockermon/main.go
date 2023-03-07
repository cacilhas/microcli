package main

import (
	"os"

	"github.com/cacilhas/daemonit"
	"github.com/cacilhas/dockermon/mon"
)

func main() {
	daemonit.DaemonIt(mon.Start, os.Args)
}
