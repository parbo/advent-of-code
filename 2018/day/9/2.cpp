#include <list>
#include <vector>
#include <iostream>

int main(int argc, char *argv[]) {
  auto players = std::atoi(argv[1]);
  auto marbles = std::atoi(argv[2]);
  std::list<int64_t> circle;
  std::vector<int64_t> score;
  score.resize(players);
  circle.push_back(0);
  auto it = circle.begin();
  auto player = 1;
  for (int marble = 1; marble < marbles + 1; ++marble) {
    if ((marble % 23) == 0) {
      for (int x = 0; x < 7; ++x) {
        if (it == circle.begin()) {
          it = circle.end();
        }
        --it;
      }
      auto value = *it;
      it = circle.erase(it);
      score[player] += (value + marble);
    } else {
      for (int x = 0; x < 2; ++x) {
        ++it;
        if (it == circle.end()) {
          it = circle.begin();
        }
      }
      it = circle.insert(it, marble);
    }
    player = (player + 1) % players;
    if (marble % 10000 == 0) {
      std::cout << marble << " " << (100.0 * marble) / marbles << "%\n";
    }
  }
  int64_t s = 0;
  for (auto sc : score) {
    if (sc > s) {
      s = sc;
    }
  }
  std::cout << s;
}
