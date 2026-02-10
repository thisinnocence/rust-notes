// 19_demo.cc: C++17 版本的学生管理 CLI（CRUD + search + order）。
//
// 编译：
//   g++ -std=c++17 -O2 -Wall -Wextra -pedantic src/bin/19_demo.cc -o
//   /tmp/19_demo_cc
//
// 运行：
//   /tmp/19_demo_cc
//
// 命令：
// - add <name> <age> <class>
// - list
// - remove <id>
// - mod <id> <name> <age> <class>
// - search id <id>
// - search name <name>
// - order <id|name|age|class> <asc|desc>
// - help
// - quit / exit

#include <algorithm>
#include <cstdint>
#include <iomanip>
#include <iostream>
#include <map>
#include <set>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

struct Student {
  std::uint32_t id;
  std::string name;
  std::uint8_t age;
  std::string class_name;
};

// 存储设计：
// - 主数据放在 by_id（unordered_map）里。
// - ids / name_index 是轻量索引，仅存 id，便于有序遍历和按 name 查询。
class StudentStore {
 public:
  std::uint32_t Add(const std::string& name, std::uint8_t age,
                    const std::string& class_name) {
    const std::uint32_t id = next_id_++;
    Student s{id, name, age, class_name};
    by_id_.emplace(id, s);
    ids_.insert(id);
    name_index_[name].insert(id);
    return id;
  }

  const Student* GetById(std::uint32_t id) const {
    auto it = by_id_.find(id);
    if (it == by_id_.end()) return nullptr;
    return &it->second;
  }

  std::vector<const Student*> ListById() const {
    std::vector<const Student*> out;
    out.reserve(ids_.size());
    for (std::uint32_t id : ids_) {
      auto it = by_id_.find(id);
      if (it != by_id_.end()) out.push_back(&it->second);
    }
    return out;
  }

  std::vector<const Student*> SearchByNameExact(const std::string& name) const {
    std::vector<const Student*> out;
    auto it = name_index_.find(name);
    if (it == name_index_.end()) return out;
    out.reserve(it->second.size());
    for (std::uint32_t id : it->second) {
      auto hit = by_id_.find(id);
      if (hit != by_id_.end()) out.push_back(&hit->second);
    }
    return out;
  }

  bool Remove(std::uint32_t id) {
    auto it = by_id_.find(id);
    if (it == by_id_.end()) return false;

    RemoveNameIndex(it->second.name, id);
    ids_.erase(id);
    by_id_.erase(it);
    return true;
  }

  bool Modify(std::uint32_t id, const std::string& name, std::uint8_t age,
              const std::string& class_name) {
    auto it = by_id_.find(id);
    if (it == by_id_.end()) return false;

    std::string old_name = it->second.name;
    if (old_name != name) {
      RemoveNameIndex(old_name, id);
      name_index_[name].insert(id);
    }

    it->second.name = name;
    it->second.age = age;
    it->second.class_name = class_name;
    return true;
  }

  std::vector<const Student*> Ordered(const std::string& field,
                                      const std::string& direction) const {
    std::vector<const Student*> rows;
    rows.reserve(by_id_.size());
    for (const auto& kv : by_id_) rows.push_back(&kv.second);

    auto cmp_asc = [&](const Student* a, const Student* b) {
      if (field == "id") return a->id < b->id;
      if (field == "name")
        return (a->name == b->name) ? (a->id < b->id) : (a->name < b->name);
      if (field == "age")
        return (a->age == b->age) ? (a->id < b->id) : (a->age < b->age);
      // class
      return (a->class_name == b->class_name) ? (a->id < b->id)
                                              : (a->class_name < b->class_name);
    };

    std::sort(rows.begin(), rows.end(), cmp_asc);
    if (direction == "desc") std::reverse(rows.begin(), rows.end());
    return rows;
  }

 private:
  void RemoveNameIndex(const std::string& name, std::uint32_t id) {
    auto it = name_index_.find(name);
    if (it == name_index_.end()) return;
    it->second.erase(id);
    if (it->second.empty()) name_index_.erase(it);
  }

  std::unordered_map<std::uint32_t, Student> by_id_;
  std::set<std::uint32_t> ids_;
  std::map<std::string, std::set<std::uint32_t>> name_index_;
  std::uint32_t next_id_ = 1;
};

static void PrintHelp() {
  std::cout << "commands:\n";
  std::cout << "  add <name> <age> <class>              add a student\n";
  std::cout
      << "  list                                  list all students by id\n";
  std::cout << "  remove <id>                           remove by id\n";
  std::cout << "  mod <id> <name> <age> <class>         modify by id\n";
  std::cout << "  search id <id>                        search by id\n";
  std::cout << "  search name <name>                    search by exact name\n";
  std::cout << "  order <id|name|age|class> <asc|desc>  ordered view\n";
  std::cout << "  help                                  show help\n";
  std::cout << "  quit | exit                           leave repl\n";
}

static void PrintStudents(const std::vector<const Student*>& rows) {
  if (rows.empty()) {
    std::cout << "(empty)\n";
    return;
  }
  std::cout << std::left << std::setw(4) << "id" << " " << std::setw(12)
            << "name" << " " << std::setw(4) << "age" << " " << std::setw(12)
            << "class" << "\n";
  for (const Student* s : rows) {
    std::cout << std::left << std::setw(4) << s->id << " " << std::setw(12)
              << s->name << " " << std::setw(4) << static_cast<int>(s->age)
              << " " << std::setw(12) << s->class_name << "\n";
  }
}

static bool ParseUint32(const std::string& raw, const char* label,
                        std::uint32_t* out) {
  std::istringstream iss(raw);
  std::uint64_t v = 0;
  if (!(iss >> v) || !iss.eof() || v > UINT32_MAX) {
    std::cout << "error: invalid " << label << " `" << raw << "`\n";
    return false;
  }
  *out = static_cast<std::uint32_t>(v);
  return true;
}

static bool ParseAge(const std::string& raw, std::uint8_t* out) {
  std::uint32_t v = 0;
  if (!ParseUint32(raw, "age", &v) || v > UINT8_MAX) {
    std::cout << "error: invalid age `" << raw << "`\n";
    return false;
  }
  *out = static_cast<std::uint8_t>(v);
  return true;
}

static std::vector<std::string> Split(const std::string& line) {
  std::istringstream iss(line);
  std::vector<std::string> parts;
  std::string token;
  while (iss >> token) parts.push_back(token);
  return parts;
}

// 返回 true 表示继续循环，false 表示退出。
static bool HandleCommand(const std::string& line, StudentStore* store) {
  std::vector<std::string> parts = Split(line);
  if (parts.empty()) return true;

  const std::string& cmd = parts[0];
  if (cmd == "add") {
    if (parts.size() != 4) {
      std::cout << "usage: add <name> <age> <class>\n";
      return true;
    }
    std::uint8_t age = 0;
    if (!ParseAge(parts[2], &age)) return true;
    std::uint32_t id = store->Add(parts[1], age, parts[3]);
    std::cout << "ok: added id=" << id << "\n";
    return true;
  }

  if (cmd == "list") {
    if (parts.size() != 1) {
      std::cout << "usage: list\n";
      return true;
    }
    PrintStudents(store->ListById());
    return true;
  }

  if (cmd == "remove") {
    if (parts.size() != 2) {
      std::cout << "usage: remove <id>\n";
      return true;
    }
    std::uint32_t id = 0;
    if (!ParseUint32(parts[1], "id", &id)) return true;
    if (store->Remove(id)) {
      std::cout << "ok: removed id=" << id << "\n";
    } else {
      std::cout << "error: id=" << id << " not found\n";
    }
    return true;
  }

  if (cmd == "mod") {
    if (parts.size() != 5) {
      std::cout << "usage: mod <id> <name> <age> <class>\n";
      return true;
    }
    std::uint32_t id = 0;
    std::uint8_t age = 0;
    if (!ParseUint32(parts[1], "id", &id)) return true;
    if (!ParseAge(parts[3], &age)) return true;
    if (store->Modify(id, parts[2], age, parts[4])) {
      std::cout << "ok: modified id=" << id << "\n";
    } else {
      std::cout << "error: id=" << id << " not found\n";
    }
    return true;
  }

  if (cmd == "search") {
    if (parts.size() < 3) {
      std::cout << "usage: search id <id> | search name <name>\n";
      return true;
    }
    if (parts[1] == "id") {
      if (parts.size() != 3) {
        std::cout << "usage: search id <id>\n";
        return true;
      }
      std::uint32_t id = 0;
      if (!ParseUint32(parts[2], "id", &id)) return true;
      const Student* s = store->GetById(id);
      if (s == nullptr) {
        std::cout << "(empty)\n";
      } else {
        PrintStudents({s});
      }
      return true;
    }
    if (parts[1] == "name") {
      if (parts.size() != 3) {
        std::cout << "usage: search name <name>\n";
        return true;
      }
      PrintStudents(store->SearchByNameExact(parts[2]));
      return true;
    }
    std::cout << "usage: search id <id> | search name <name>\n";
    return true;
  }

  if (cmd == "order") {
    if (parts.size() != 3) {
      std::cout << "usage: order <id|name|age|class> <asc|desc>\n";
      return true;
    }
    if (parts[1] != "id" && parts[1] != "name" && parts[1] != "age" &&
        parts[1] != "class") {
      std::cout << "error: invalid field `" << parts[1] << "`\n";
      return true;
    }
    if (parts[2] != "asc" && parts[2] != "desc") {
      std::cout << "error: invalid direction `" << parts[2] << "`\n";
      return true;
    }
    PrintStudents(store->Ordered(parts[1], parts[2]));
    return true;
  }

  if (cmd == "help") {
    PrintHelp();
    return true;
  }

  if (cmd == "quit" || cmd == "exit") {
    return false;
  }

  std::cout << "unknown command. type `help`\n";
  return true;
}

int main() {
  StudentStore store;

  std::cout << "student-cli demo (cpp17)\n";
  std::cout << "type `help` to see commands\n";

  std::string line;
  while (true) {
    std::cout << "sms> " << std::flush;
    if (!std::getline(std::cin, line)) {
      std::cout << "\n";
      break;
    }
    if (line.empty()) continue;
    if (!HandleCommand(line, &store)) break;
  }

  std::cout << "bye\n";
  return 0;
}
