package main

import (
	"net/http"
	"teachertools/grouping/internal/net"

	"github.com/charmbracelet/log"
	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
)

func main() {
	log.SetLevel(log.DebugLevel)
	r := chi.NewRouter()
	r.Use(middleware.Logger)
	r.Get("/", func(w http.ResponseWriter, r *http.Request) {
		w.Write([]byte("welcome"))
	})
	r.Get("/group", net.GetNewGroup)
	http.ListenAndServe(":3000", r)
}
