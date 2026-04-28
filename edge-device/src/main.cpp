#include <chrono>
#include <cmath>
#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <iomanip>
#include <iostream>
#include <random>
#include <sstream>
#include <string>
#include <thread>
#include <vector>

#ifdef _WIN32
#include <winsock2.h>
#include <ws2tcpip.h>
using socket_handle_t = SOCKET;
#else
#include <arpa/inet.h>
#include <netinet/in.h>
#include <sys/socket.h>
#include <unistd.h>
using socket_handle_t = int;
#endif

namespace {

constexpr double kPi = 3.14159265358979323846;

struct TelemetryFrame {
    std::string node_id;
    double x;
    double y;
    double z;
    double temperature_c;
    int signal_dbm;
    std::string status;
};

std::string getenv_or_default(const char* name, const char* fallback) {
    const char* value = std::getenv(name);
    return value != nullptr ? std::string(value) : std::string(fallback);
}

uint16_t getenv_port(const char* name, uint16_t fallback) {
    const char* value = std::getenv(name);
    if (value == nullptr) {
        return fallback;
    }

    const long parsed = std::strtol(value, nullptr, 10);
    if (parsed <= 0 || parsed > 65535) {
        return fallback;
    }

    return static_cast<uint16_t>(parsed);
}

class UdpBroadcaster {
public:
    UdpBroadcaster(const std::string& host, uint16_t port) : socket_(invalid_socket()) {
#ifdef _WIN32
        WSADATA data;
        if (WSAStartup(MAKEWORD(2, 2), &data) != 0) {
            throw std::runtime_error("failed to initialize winsock");
        }
#endif

        socket_ = ::socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
        if (socket_ == invalid_socket()) {
            cleanup_runtime();
            throw std::runtime_error("failed to create udp socket");
        }

        std::memset(&address_, 0, sizeof(address_));
        address_.sin_family = AF_INET;
        address_.sin_port = htons(port);

#ifdef _WIN32
        if (InetPtonA(AF_INET, host.c_str(), &address_.sin_addr) != 1) {
            close_socket();
            throw std::runtime_error("failed to parse destination host");
        }
#else
        if (inet_pton(AF_INET, host.c_str(), &address_.sin_addr) != 1) {
            close_socket();
            throw std::runtime_error("failed to parse destination host");
        }
#endif
    }

    ~UdpBroadcaster() {
        close_socket();
    }

    bool send(const std::string& payload) {
        const auto* bytes = payload.c_str();
        const auto size = static_cast<int>(payload.size());
        const int sent = ::sendto(
            socket_,
            bytes,
            size,
            0,
            reinterpret_cast<const sockaddr*>(&address_),
            static_cast<int>(sizeof(address_)));

        return sent == size;
    }

private:
    socket_handle_t socket_;
    sockaddr_in address_ {};

    static socket_handle_t invalid_socket() {
#ifdef _WIN32
        return INVALID_SOCKET;
#else
        return -1;
#endif
    }

    void close_socket() {
        if (socket_ == invalid_socket()) {
            cleanup_runtime();
            return;
        }

#ifdef _WIN32
        closesocket(socket_);
#else
        close(socket_);
#endif
        socket_ = invalid_socket();
        cleanup_runtime();
    }

    void cleanup_runtime() {
#ifdef _WIN32
        WSACleanup();
#endif
    }
};

TelemetryFrame build_frame(int index, int tick, std::mt19937& random) {
    const double base_angle = (static_cast<double>(index) * 0.4) + (static_cast<double>(tick) * 0.15);
    const double radius = 10.0 + (index % 6) * 2.5;
    std::uniform_real_distribution<double> noise(-0.25, 0.25);

    TelemetryFrame frame;
    frame.node_id = "sensor-" + std::to_string(index + 1);
    frame.x = std::cos(base_angle) * radius + noise(random);
    frame.y = ((index % 5) - 2) * 0.8;
    frame.z = std::sin(base_angle) * radius + noise(random);
    frame.temperature_c = 17.0 + (index % 7) * 1.8 + std::sin(base_angle * 0.5);
    frame.signal_dbm = -42 - ((index * 3 + tick) % 28);
    frame.status = frame.signal_dbm < -62 ? "warning" : "nominal";
    return frame;
}

std::string serialize_frame(const TelemetryFrame& frame) {
    std::ostringstream stream;
    stream << std::fixed << std::setprecision(3)
           << "{"
           << "\"node_id\":\"" << frame.node_id << "\"," 
           << "\"x\":" << frame.x << ","
           << "\"y\":" << frame.y << ","
           << "\"z\":" << frame.z << ","
           << "\"temperature_c\":" << frame.temperature_c << ","
           << "\"signal_dbm\":" << frame.signal_dbm << ","
           << "\"status\":\"" << frame.status << "\""
           << "}";
    return stream.str();
}

}  // namespace

int main(int argc, char** argv) {
    const std::string host = getenv_or_default("SPATIAL_IOT_UDP_HOST", "127.0.0.1");
    const uint16_t port = getenv_port("SPATIAL_IOT_UDP_PORT", 7001);
    const bool run_once = argc > 1 && std::string(argv[1]) == "--once";

    try {
        UdpBroadcaster broadcaster(host, port);
        std::mt19937 random(1337);
        const int tick_limit = run_once ? 1 : 120;
        const int node_count = 18;

        std::cout << "broadcasting telemetry to udp://" << host << ':' << port << std::endl;

        for (int tick = 0; tick < tick_limit; ++tick) {
            for (int index = 0; index < node_count; ++index) {
                const auto frame = build_frame(index, tick, random);
                const auto payload = serialize_frame(frame);
                if (!broadcaster.send(payload)) {
                    std::cerr << "failed to send payload for " << frame.node_id << std::endl;
                }
            }

            std::cout << "tick " << tick + 1 << " dispatched " << node_count << " telemetry packets" << std::endl;
            std::this_thread::sleep_for(std::chrono::milliseconds(250));
        }
    } catch (const std::exception& error) {
        std::cerr << error.what() << std::endl;
        return 1;
    }

    return 0;
}
