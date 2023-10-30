FROM golang:1.19.2 as builder
WORKDIR /app
COPY . .
RUN go build -o bin/gofiber src/main.go

FROM gcr.io/distroless/cc-debian11
WORKDIR /app
COPY --from=builder /app/bin/gofiber .
EXPOSE 5041
CMD ["./gofiber"]