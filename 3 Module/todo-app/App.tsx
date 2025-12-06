import { StatusBar } from 'expo-status-bar';
import { StyleSheet, Text } from 'react-native';
import TodoList from './components/TodoList';
import { SafeAreaView } from 'react-native-safe-area-context';
import { initDatabase } from './services/database';


export default function App() {
  initDatabase();

  return (
    <SafeAreaView style={styles.container}>
      <TodoList />
      <StatusBar style="auto" />
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    backgroundColor: '#fff',
  },
});
