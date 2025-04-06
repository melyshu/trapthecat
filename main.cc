#include <cstdint>
#include <array>
#include <iostream>
#include <vector>
#include <set>
#include <map>
#include <optional>
#include <fstream>
#include <queue>

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

    static const std::vector<Position> edges;

    std::vector<Position> neighbors() const
    {
        std::vector<Position> ret;

        const auto maybe_insert = [&ret](Position p)
        {
            if (p.is_valid())
            {
                ret.push_back(p);
            }
        };

        maybe_insert(Position(r, c - 1));
        maybe_insert(Position(r - 1, c - ((r + 1) % 2)));
        maybe_insert(Position(r - 1, c - ((r + 1) % 2) + 1));
        maybe_insert(Position(r, c + 1));
        maybe_insert(Position(r + 1, c - ((r + 1) % 2) + 1));
        maybe_insert(Position(r + 1, c - ((r + 1) % 2)));

        return ret;
    }

    friend std::ostream &operator<<(std::ostream &os, const Position &pos)
    {
        return os << "Position(r=" << std::to_string(pos.r)
                  << ", c=" << std::to_string(pos.c) << ")";
    }

    friend std::istream &operator>>(std::istream &is, Position &pos)
    {
        int r, c;
        is >> r >> c;
        pos.r = r;
        pos.c = c;
        return is;
    }
};

const std::vector<Position> Position::edges = []()
{
    std::vector<Position> ret;

    for (int_t r = 0; r < R; ++r)
    {
        for (int_t c = 0; c < C; ++c)
        {
            Position p(r, c);
            if (p.is_edge())
            {
                ret.push_back(p);
            }
        }
    }
    return ret;
}();

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

    std::optional<Position> move_cat()
    {
        std::map<Position, int> pos2num_paths;
        std::set<Position> layer;
        for (const auto &p : Position::edges)
        {
            pos2num_paths[p] = cells[p.idx()] ? 0 : 1;
            if (!cells[p.idx()])
            {
                layer.insert(p);
            }
        }

        while (true)
        {
            if (layer.empty())
            {
                // Cat not found
                return std::nullopt;
            }

            if (layer.count(cat_position) > 0)
            {
                int best_num_paths = 0;
                Position best_move;
                for (const auto p : cat_position.neighbors())
                {
                    if (layer.count(p) > 0)
                    {
                        continue;
                    }

                    if (auto it = pos2num_paths.find(p);
                        it != pos2num_paths.end() && it->second > best_num_paths)
                    {
                        best_move = it->first;
                        best_num_paths = it->second;
                    }
                }

                if (best_num_paths == 0)
                {
                    throw std::runtime_error("sth bad happened oops");
                }

                cat_position = best_move;
                return best_move;
            }

            std::set<Position> next_layer;
            for (const auto &p : layer)
            {
                for (const auto &pp : p.neighbors())
                {
                    if (cells[pp.idx()])
                    {
                        continue;
                    }

                    if (next_layer.count(pp) || !pos2num_paths.count(pp))
                    {
                        pos2num_paths[pp] += pos2num_paths[p];
                        next_layer.insert(pp);
                    }
                }
            }

            layer = next_layer;
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

    friend std::istream &operator>>(std::istream &is, Board &board)
    {
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

                    board.cat_position = Position(r, c);
                    cat_found = true;
                    board.cells[p.idx()] = false;
                }
                else if (s == '#')
                {
                    board.cells[p.idx()] = true;
                }
                else if (s == '-')
                {
                    board.cells[p.idx()] = false;
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

        return is;
    }

    friend std::ostream &operator<<(std::ostream &os, const Board &board)
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

                if (board.cat_position == p)
                {
                    os << 'C';
                }
                else if (board.cells[p.idx()])
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

        return os;
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

        Position pos;
        std::cin >> pos;
        return pos;
    }
};

struct AutoPlayer : BasePlayer
{
    std::deque<Position> cached_moves;

    static int_t get_min_cat_moves(const Board &board)
    {
        std::set<Position> visited;
        std::set<Position> current;

        visited.insert(board.cat_position);
        current.insert(board.cat_position);

        int_t num_cat_moves = 0;
        while (true)
        {
            if (current.empty())
            {
                return N;
            }

            for (const auto p : current)
            {
                if (p.is_edge())
                {
                    return num_cat_moves;
                }
            }

            std::set<Position> new_current;
            for (const auto p : current)
            {
                for (const auto pp : p.neighbors())
                {
                    if (!visited.count(pp) && !board.cells[pp.idx()])
                    {
                        new_current.insert(pp);
                    }
                }
            }

            current = new_current;
            visited.insert(current.begin(), current.end());
            ++num_cat_moves;
        }
    }

    struct SearchState
    {
        Board board;
        mutable int_t cached_min_cat_moves = -1;
        int_t num_moves = 0;
        std::shared_ptr<SearchState> previous;
        std::optional<Position> move;

        int_t min_cat_moves() const
        {
            if (cached_min_cat_moves == -1)
            {
                cached_min_cat_moves = get_min_cat_moves(board);
            }

            return cached_min_cat_moves;
        }

        int priority() const
        {
            return min_cat_moves();
        }

        bool operator<(const SearchState &other) const
        {
            return priority() < other.priority();
        }

        friend std::ostream &operator<<(std::ostream &os, const SearchState &search_state)
        {
            os << "State(" << std::endl;
            os << "  priority=" << search_state.priority() << "," << std::endl;
            os << "  num_moves=" << std::to_string(search_state.num_moves) << "," << std::endl;
            os << "  previous=" << (search_state.previous ? "yes" : "no") << "," << std::endl;
            ;
            os << "  move=";
            if (search_state.move)
            {
                os << *search_state.move;
            }
            else
            {
                os << "none";
            }
            os << "," << std::endl;
            os << "  min_cat_moves=" << std::to_string(search_state.min_cat_moves()) << "," << std::endl;
            os << ") with board:" << std::endl;
            os << search_state.board;
            return os;
        }
    };

    Position get_move(const Board &board) override
    {
        if (!cached_moves.empty())
        {
            auto ret = cached_moves.back();
            cached_moves.pop_back();
            return ret;
        }

        std::priority_queue<SearchState> pq;
        SearchState first_state;
        first_state.board = board;
        pq.emplace(first_state);

        bool first = true;

        while (true)
        {
            std::clog << "Popping top of " << pq.size() << " states: " << std::endl;
            auto current_ptr = std::make_shared<SearchState>(pq.top());
            pq.pop();
            auto &current_state = *current_ptr;

            if (first)
            {
                first = false;
            }
            else
            {
                current_state.board.move_cat();
            }

            std::clog << current_state << std::endl;

            if (current_state.cached_min_cat_moves == N)
            {
                std::clog << "Solution found!" << std::endl;
                for (auto ptr = current_ptr; ptr->move && ptr->previous; ptr = ptr->previous)
                {
                    cached_moves.push_back(*ptr->move);
                }

                auto ret = cached_moves.back();
                cached_moves.pop_back();
                return ret;
            }

            for (int_t r = 0; r < R; ++r)
            {
                for (int_t c = 0; c < C; ++c)
                {
                    Position p(r, c);
                    if (!current_state.board.cells[p.idx()] && !(p == current_state.board.cat_position))
                    {
                        SearchState next_state;
                        Board next_board = current_state.board;
                        next_board.cells[p.idx()] = true;
                        next_state.board = next_board;
                        next_state.num_moves = current_state.num_moves + 1;
                        next_state.previous = current_ptr;
                        next_state.move = p;

                        pq.push(next_state);
                    }
                }
            }
        }
    }
};

int main(int argc, char *argv[])
{
    if (argc < 2)
    {
        std::clog << "Usage: " << argv[0] << " <input_file>" << std::endl;
        return 1;
    }

    Board board;
    std::ifstream ifs(argv[1]);
    ifs >> board;

    std::cout << board << std::endl;

    // std::unique_ptr<BasePlayer> player = std::make_unique<InteractivePlayer>();
    std::unique_ptr<BasePlayer> player = std::make_unique<AutoPlayer>();

    while (true)
    {
        if (board.cat_position.is_edge())
        {
            std::cout << "Failed! The cat ran away!" << std::endl;
            break;
        }

        while (true)
        {
            const auto move = player->get_move(board);
            if (board.move_player(move))
            {
                std::cout << "Accepted player's move: " << move << std::endl;
                break;
            }
        }

        std::cout << board << std::endl;

        if (const auto move = board.move_cat())
        {
            std::cout << "Cat moved: " << *move << std::endl;
            std::cout << board << std::endl;
        }
        else
        {
            std::cout << "Succeeded! You trapped the cat!" << std::endl;
            break;
        }
    }
}
