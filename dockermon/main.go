package main

import (
	"os"

	"github.com/cacilhas/daemonit"
	"github.com/cacilhas/microcli/dockermon/mon"
)

func main() {
	daemonit.DaemonIt(mon.Start, os.Args)
}
