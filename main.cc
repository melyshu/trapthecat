#include <cstdint>
#include <array>
#include <iostream>
#include <vector>
#include <set>
#include <map>
#include <optional>
#include <fstream>

using int_t = int8_t;
constexpr int_t R = 11;
constexpr int_t C = 10;
constexpr int_t N = R * C;

struct Position
{
    int_t r;
    int_t c;

    Position(int r_ = 0, int c_ = 0) : r(r_), c(c_) {}

    int_t idx() const
    {
        return r * C + c;
    }

    bool operator==(Position other) const
    {
        return r == other.r && c == other.c;
    }

    bool operator<(Position other) const
    {
        return idx() < other.idx();
    }

    bool is_valid() const
    {
        return 0 <= r && r < R && 0 <= c && c < C;
    }

    bool is_edge() const
    {
        return r == 0 || r == R - 1 || c == 0 || c == C - 1;
    }

    std::set<Position> neighbors() const
    {
        std::set<Position> ret;

        const auto maybe_insert = [&ret](Position p)
        {
            if (p.is_valid())
            {
                ret.insert(p);
            }
        };

        maybe_insert(Position(r - 1, c - ((r + 1) % 2)));
        maybe_insert(Position(r - 1, c - ((r + 1) % 2) + 1));
        maybe_insert(Position(r, c - 1));
        maybe_insert(Position(r, c + 1));
        maybe_insert(Position(r + 1, c - ((r + 1) % 2)));
        maybe_insert(Position(r + 1, c - ((r + 1) % 2) + 1));

        return ret;
    }
};

void throw_error(char s, int_t r, int_t c, int line_no)
{
    throw std::runtime_error(std::string("Unexpected on line ") + std::to_string(line_no) +
                             std::string(": ") + s +
                             std::string(" at r=") + std::to_string(r) +
                             std::string(" c=") + std::to_string(c));
}

struct Board
{
    std::bitset<N> cells;
    Position cat_position;

    static Board load(std::istream &is)
    {
        Board ret;
        char s;
        bool cat_found = false;

        for (int_t r = 0; r < R; ++r)
        {
            for (int_t c = 0; c < C; ++c)
            {
                Position p(r, c);

                is >> s;
                if (s == 'C')
                {
                    if (cat_found)
                    {
                        throw_error(s, r, c, __LINE__);
                    }

                    ret.cat_position = Position(r, c);
                    cat_found = true;
                    ret.cells[p.idx()] = false;
                }
                else if (s == '#')
                {
                    ret.cells[p.idx()] = true;
                }
                else if (s == '-')
                {
                    ret.cells[p.idx()] = false;
                }
                else
                {
                    throw_error(s, r, c, __LINE__);
                }
            }
        }

        if (!cat_found)
        {
            throw_error('C', -1, -1, __LINE__);
        }

        return ret;
    }

    void dump(std::ostream &os) const
    {
        for (int_t r = 0; r < R; ++r)
        {
            if (r % 2)
            {
                os << ' ';
            }

            for (int_t c = 0; c < C; ++c)
            {
                Position p(r, c);

                if (c)
                {
                    os << ' ';
                }

                if (cat_position == p)
                {
                    os << 'C';
                }
                else if (cells[p.idx()])
                {
                    os << '#';
                }
                else
                {
                    os << '-';
                }
            }

            os << std::endl;
        }

        os << std::endl;
    }

    bool move_cat()
    {
        struct SearchState
        {
            std::bitset<N> visited;
            std::map<Position, int> pos2num_paths;

            bool exhausted() const
            {
                return pos2num_paths.empty();
            }

            int num_edge_paths() const
            {
                int ret = 0;
                for (const auto [p, num_paths] : pos2num_paths)
                {
                    if (p.is_edge())
                    {
                        ret += num_paths;
                    }
                }

                return ret;
            }

            void take_step(const std::bitset<N> &cells)
            {
                std::map<Position, int> new_pos2num_paths;

                for (const auto [p, num_paths] : pos2num_paths)
                {
                    for (const auto pp : p.neighbors())
                    {
                        if (!cells[pp.idx()] && !visited[pp.idx()])
                        {
                            new_pos2num_paths[pp] += num_paths;
                        }
                    }
                }

                pos2num_paths = new_pos2num_paths;
                for (const auto [p, _] : pos2num_paths)
                {
                    visited[p.idx()] = true;
                }
            }
        };

        std::map<Position, SearchState> moves2search_state;
        for (const auto p : cat_position.neighbors())
        {
            if (!cells[p.idx()])
            {
                auto &search_state = moves2search_state[p];
                search_state.visited[cat_position.idx()] = true;
                search_state.visited[p.idx()] = true;
                search_state.pos2num_paths[p] = 1;
            }
        }

        while (true)
        {
            if (moves2search_state.empty())
            {
                return false;
            }

            std::optional<Position> best_move;
            int most_edge_paths = 0;
            for (const auto &[move, search_state] : moves2search_state)
            {
                const auto num_edge_paths = search_state.num_edge_paths();
                if (num_edge_paths > most_edge_paths)
                {
                    best_move = move;
                    most_edge_paths = num_edge_paths;
                }
            }

            if (best_move.has_value())
            {
                cat_position = *best_move;
                return true;
            }

            auto it = moves2search_state.begin();
            while (it != moves2search_state.end())
            {
                auto &search_state = it->second;
                search_state.take_step(cells);
                if (search_state.exhausted())
                {
                    it = moves2search_state.erase(it);
                }
                else
                {
                    ++it;
                }
            }
        }
    }

    bool move_player(Position p)
    {
        if (p.is_valid() && !cells[p.idx()] && !(p == cat_position))
        {
            cells[p.idx()] = true;
            return true;
        }

        return false;
    }
};

struct BasePlayer
{
    virtual Position get_move(const Board &board) = 0;
    virtual ~BasePlayer() {};
};

struct InteractivePlayer : BasePlayer
{
    Position get_move(const Board &board) override
    {
        std::cout << "Please enter your move (r c):" << std::endl;

        int r, c;
        std::cin >> r >> c;
        return Position(r, c);
    }
};

struct AutoPlayer : BasePlayer
{
    Position get_move(const Board &board) override
    {
        // To... uhh... make smarter.
        return Position(0, 0);
    }
};

int main(int argc, char *argv[])
{
    if (argc < 2)
    {
        std::clog << "Usage: " << argv[0] << " <input_file>" << std::endl;
        return 1;
    }

    auto board = [file_path = argv[1]]()
    {
        std::ifstream ifs(file_path);
        return Board::load(ifs);
    }();

    board.dump(std::cout);

    std::unique_ptr<BasePlayer> player = std::make_unique<InteractivePlayer>();

    while (true)
    {
        if (board.cat_position.is_edge())
        {
            std::cout << "Failed! The cat ran away!" << std::endl;
            break;
        }

        while (true)
        {
            if (board.move_player(player->get_move(board)))
            {
                break;
            }
        }

        board.dump(std::cout);

        if (board.move_cat())
        {
            std::cout << "Cat moved:" << std::endl;
            board.dump(std::cout);
        }
        else
        {
            std::cout << "Succeeded! You trapped the cat!" << std::endl;
            break;
        }
    }
}
